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
    pub mic_muted: bool,
    pub audio_device_detected: bool,
    pub speaker_volume: u8,
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
    mic_muted: RwLock<bool>,
    prev_mic_btn: RwLock<bool>,
    audio_device_detected: RwLock<bool>,
    speaker_volume: RwLock<u8>,
    mic_toggle_callback: RwLock<Option<Arc<dyn Fn(bool) + Send + Sync>>>,
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
            mic_muted: RwLock::new(false),
            prev_mic_btn: RwLock::new(false),
            audio_device_detected: RwLock::new(true),
            speaker_volume: RwLock::new(80),
            mic_toggle_callback: RwLock::new(None),
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
        let mic_muted = *self.mic_muted.read();
        let audio_device_detected = *self.audio_device_detected.read();
        let speaker_volume = *self.speaker_volume.read();

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
            mic_muted,
            audio_device_detected,
            speaker_volume,
        }
    }

    pub fn toggle_microphone(&self) -> bool {
        let mut muted = self.mic_muted.write();
        *muted = !*muted;
        let is_muted = *muted;

        let mut lighting = self.lighting.write();
        lighting.mic_led = if is_muted {
            crate::lightbar::MicLed::Solid
        } else {
            crate::lightbar::MicLed::Off
        };

        is_muted
    }

    pub fn set_microphone_mute(&self, muted: bool) {
        *self.mic_muted.write() = muted;
        let mut lighting = self.lighting.write();
        lighting.mic_led = if muted {
            crate::lightbar::MicLed::Solid
        } else {
            crate::lightbar::MicLed::Off
        };
    }

    pub fn set_speaker_volume(&self, volume: u8) {
        *self.speaker_volume.write() = volume.min(100);
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

    pub fn get_triggers(&self) -> DualTriggerConfig {
        self.shared_state.triggers.read().clone()
    }

    pub fn get_lighting(&self) -> LightingConfig {
        self.lighting.read().clone()
    }

    pub fn set_connected(&self, connected: bool, name: &str) {
        *self.connected.write() = connected;
        *self.controller_name.write() = name.to_string();
    }

    pub fn set_mic_toggle_handler(&self, handler: Arc<dyn Fn(bool) + Send + Sync>) {
        *self.mic_toggle_callback.write() = Some(handler);
    }

    pub fn update_input_state(&self, new_state: DualSenseInputState) -> Option<bool> {
        let mut toggled_to = None;
        let prev_pressed = *self.prev_mic_btn.read();
        let curr_pressed = new_state.mic_mute;
        if curr_pressed && !prev_pressed {
            // Physical button pressed edge
            let new_muted = self.toggle_microphone();
            if let Some(cb) = self.mic_toggle_callback.read().as_ref() {
                cb(new_muted);
            }
            toggled_to = Some(new_muted);
        }
        *self.prev_mic_btn.write() = curr_pressed;
        *self.input_state.write() = new_state;
        toggled_to
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

        // Test Microphone Mute toggle and LED sync
        assert!(!manager.get_status().await.mic_muted);
        let toggled = manager.toggle_microphone();
        assert!(toggled);
        assert!(manager.get_status().await.mic_muted);
        assert_eq!(
            manager.get_status().await.lighting.mic_led,
            crate::lightbar::MicLed::Solid
        );

        let toggled_back = manager.toggle_microphone();
        assert!(!toggled_back);
        assert!(!manager.get_status().await.mic_muted);
        assert_eq!(
            manager.get_status().await.lighting.mic_led,
            crate::lightbar::MicLed::Off
        );

        // Test physical button edge detection
        let mut input_press = DualSenseInputState::default();
        input_press.mic_mute = true;
        let event = manager.update_input_state(input_press);
        assert_eq!(event, Some(true)); // Toggled to muted
        assert!(manager.get_status().await.mic_muted);

        // While held down, should not trigger again
        let mut input_hold = DualSenseInputState::default();
        input_hold.mic_mute = true;
        let event_hold = manager.update_input_state(input_hold);
        assert_eq!(event_hold, None);

        // Released
        let mut input_release = DualSenseInputState::default();
        input_release.mic_mute = false;
        let event_rel = manager.update_input_state(input_release);
        assert_eq!(event_rel, None);

        // Press again -> unmutes
        let mut input_press2 = DualSenseInputState::default();
        input_press2.mic_mute = true;
        let event2 = manager.update_input_state(input_press2);
        assert_eq!(event2, Some(false)); // Toggled to unmuted
        assert!(!manager.get_status().await.mic_muted);
    }
}
