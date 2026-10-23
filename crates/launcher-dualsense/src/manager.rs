use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::emulation::EmulationConfig;
use crate::lightbar::LightingConfig;
use crate::profiles::TriggerProfile;
use crate::protocol::{BatteryState, ConnectionType, DualSenseInputState, OutputReportBuilder};
use crate::triggers::DualTriggerConfig;
use crate::udp_server::{
    DualSenseUdpServer, SharedControllerState, UdpServerStats, DEFAULT_DUALSENSEX_UDP_PORT,
};

/// Comprehensive controller status report for UI and API clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualSenseStatus {
    pub connected: bool,
    pub name: String,
    pub connection_type: ConnectionType,
    pub battery: BatteryState,
    pub triggers: DualTriggerConfig,
    pub lighting: LightingConfig,
    pub emulation: EmulationConfig,
    pub udp_stats: UdpServerStats,
    pub profiles: Vec<TriggerProfile>,
}

pub struct DualSenseManager {
    shared_state: Arc<SharedControllerState>,
    lighting: RwLock<LightingConfig>,
    emulation: RwLock<EmulationConfig>,
    input_state: RwLock<DualSenseInputState>,
    connected: RwLock<bool>,
    controller_name: RwLock<String>,
    udp_server: tokio::sync::Mutex<DualSenseUdpServer>,
    profiles: RwLock<Vec<TriggerProfile>>,
}

impl DualSenseManager {
    pub fn new() -> Self {
        let shared_state = Arc::new(SharedControllerState::default());
        let udp_server = DualSenseUdpServer::new(DEFAULT_DUALSENSEX_UDP_PORT, shared_state.clone());

        let initial_input = DualSenseInputState {
            connection: ConnectionType::Usb,
            battery: BatteryState {
                percentage: 90,
                is_charging: false,
                is_full: false,
            },
            ..Default::default()
        };

        Self {
            shared_state,
            lighting: RwLock::new(LightingConfig::default()),
            emulation: RwLock::new(EmulationConfig::default()),
            input_state: RwLock::new(initial_input),
            connected: RwLock::new(true),
            controller_name: RwLock::new("Sony DualSense Wireless Controller".to_string()),
            udp_server: tokio::sync::Mutex::new(udp_server),
            profiles: RwLock::new(TriggerProfile::default_profiles()),
        }
    }

    pub async fn start_udp(&self) -> Result<(), std::io::Error> {
        let mut server = self.udp_server.lock().await;
        server.start().await
    }

    pub async fn stop_udp(&self) {
        let mut server = self.udp_server.lock().await;
        server.stop();
    }

    pub async fn get_status(&self) -> DualSenseStatus {
        let udp_stats = {
            let server = self.udp_server.lock().await;
            server.stats()
        };

        let input = self.input_state.read().clone();
        let triggers = self.shared_state.triggers.read().clone();
        let lighting = self.lighting.read().clone();
        let emulation = self.emulation.read().clone();
        let connected = *self.connected.read();
        let name = self.controller_name.read().clone();
        let profiles = self.profiles.read().clone();

        DualSenseStatus {
            connected,
            name,
            connection_type: input.connection,
            battery: input.battery,
            triggers,
            lighting,
            emulation,
            udp_stats,
            profiles,
        }
    }

    pub fn set_triggers(&self, triggers: DualTriggerConfig) {
        *self.shared_state.triggers.write() = triggers;
    }

    pub fn set_lighting(&self, lighting: LightingConfig) {
        if let crate::lightbar::LightbarMode::Static { color } = lighting.lightbar {
            *self.shared_state.lightbar.write() = color;
        }
        *self.lighting.write() = lighting;
    }

    pub fn set_emulation(&self, config: EmulationConfig) {
        *self.emulation.write() = config;
    }

    pub fn apply_profile(&self, profile_id: &str) -> bool {
        let profiles = self.profiles.read();
        if let Some(p) = profiles.iter().find(|p| p.id == profile_id) {
            *self.shared_state.triggers.write() = p.triggers.clone();
            *self.shared_state.lightbar.write() = p.lightbar;
            let mut lighting = self.lighting.write();
            lighting.lightbar = crate::lightbar::LightbarMode::Static { color: p.lightbar };
            true
        } else {
            false
        }
    }

    /// Generates current output report for physical controller transmission
    pub fn build_current_output_report(&self) -> OutputReportBuilder {
        let triggers = self.shared_state.triggers.read().clone();
        let lighting = self.lighting.read().clone();
        let lightbar_color = *self.shared_state.lightbar.read();
        let heavy = *self.shared_state.heavy_rumble.read();
        let light = *self.shared_state.light_rumble.read();

        OutputReportBuilder::new()
            .with_triggers(triggers)
            .with_lightbar(lightbar_color)
            .with_player_led(lighting.player_led)
            .with_mic_led(lighting.mic_led)
            .with_rumble(heavy, light)
    }

    pub fn update_input_state(&self, new_state: DualSenseInputState) {
        *self.input_state.write() = new_state;
    }
}

impl Default for DualSenseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lightbar::RgbColor;
    use crate::triggers::TriggerEffect;

    #[tokio::test]
    async fn test_manager_status_and_updates() {
        let manager = DualSenseManager::new();
        let status = manager.get_status().await;
        assert!(status.connected);
        assert_eq!(status.battery.percentage, 90);

        // Update triggers
        manager.set_triggers(DualTriggerConfig {
            left: TriggerEffect::Feedback {
                start_position: 1,
                force: 4,
            },
            right: TriggerEffect::Weapon {
                start_position: 2,
                end_position: 6,
                force: 7,
            },
        });

        let updated_status = manager.get_status().await;
        assert_eq!(
            updated_status.triggers.right,
            TriggerEffect::Weapon {
                start_position: 2,
                end_position: 6,
                force: 7
            }
        );

        // Apply profile
        let ok = manager.apply_profile("racing_abs");
        assert!(ok);
        assert!(!manager.apply_profile("unknown_id"));

        let racing_status = manager.get_status().await;
        assert_eq!(
            racing_status.lighting.lightbar,
            crate::lightbar::LightbarMode::Static {
                color: RgbColor::RED
            }
        );

        // Test output report builder
        let report = manager.build_current_output_report();
        let usb = report.build_usb();
        assert_eq!(usb[0], 0x02);
    }
}
