use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

use crate::lightbar::RgbColor;
use crate::triggers::{DualTriggerConfig, TriggerEffect};

pub const DEFAULT_DUALSENSEX_UDP_PORT: u16 = 6969;

/// DualSenseX instruction type codes (used in DualSenseX C# / C++ link mods)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DualSenseXInstructionType {
    TriggerUpdate = 1,
    RgbUpdate = 2,
    PlayerLedUpdate = 3,
    RumbleUpdate = 4,
    Reset = 5,
    PlayerLedNewRevision = 6,
    MicLed = 7,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualSenseXInstruction {
    #[serde(rename = "type", alias = "Type")]
    pub inst_type: u32,
    #[serde(default, alias = "Parameters")]
    pub parameters: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualSenseXPacket {
    #[serde(default, alias = "Instructions")]
    pub instructions: Vec<DualSenseXInstruction>,
    #[serde(default)]
    pub left_trigger: Option<TriggerEffect>,
    #[serde(default)]
    pub right_trigger: Option<TriggerEffect>,
    #[serde(default)]
    pub rgb: Option<RgbColor>,
}

/// Statistics and health of the DualSenseX UDP link server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpServerStats {
    pub running: bool,
    pub port: u16,
    pub packets_received: u64,
    pub last_packet_time: Option<i64>,
    pub last_sender: Option<String>,
}

/// Thread-safe controller state shared with the UDP server
#[derive(Debug, Default)]
pub struct SharedControllerState {
    pub triggers: RwLock<DualTriggerConfig>,
    pub lightbar: RwLock<RgbColor>,
    pub heavy_rumble: RwLock<u8>,
    pub light_rumble: RwLock<u8>,
}

pub struct DualSenseUdpServer {
    port: u16,
    is_running: Arc<AtomicBool>,
    packets_received: Arc<AtomicU64>,
    last_packet_time: Arc<RwLock<Option<i64>>>,
    last_sender: Arc<RwLock<Option<String>>>,
    shared_state: Arc<SharedControllerState>,
    worker_handle: Option<JoinHandle<()>>,
}

impl DualSenseUdpServer {
    pub fn new(port: u16, shared_state: Arc<SharedControllerState>) -> Self {
        Self {
            port,
            is_running: Arc::new(AtomicBool::new(false)),
            packets_received: Arc::new(AtomicU64::new(0)),
            last_packet_time: Arc::new(RwLock::new(None)),
            last_sender: Arc::new(RwLock::new(None)),
            shared_state,
            worker_handle: None,
        }
    }

    pub fn stats(&self) -> UdpServerStats {
        UdpServerStats {
            running: self.is_running.load(Ordering::SeqCst),
            port: self.port,
            packets_received: self.packets_received.load(Ordering::SeqCst),
            last_packet_time: *self.last_packet_time.read(),
            last_sender: self.last_sender.read().clone(),
        }
    }

    pub async fn start(&mut self) -> Result<(), std::io::Error> {
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(());
        }

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let socket = UdpSocket::bind(addr).await?;
        self.is_running.store(true, Ordering::SeqCst);

        // Write %TEMP%\DualSenseX\DualSenseX_PortNumber.txt for Paliverse/DualSenseX mod discovery
        let temp_dsx_dir = std::env::temp_dir().join("DualSenseX");
        if let Err(e) = std::fs::create_dir_all(&temp_dsx_dir) {
            tracing::debug!("Could not create temp DualSenseX dir: {}", e);
        } else {
            let port_file = temp_dsx_dir.join("DualSenseX_PortNumber.txt");
            if let Err(e) = std::fs::write(&port_file, self.port.to_string()) {
                tracing::debug!("Could not write DualSenseX_PortNumber.txt: {}", e);
            } else {
                tracing::info!(
                    "Published DualSenseX port discovery file at {}",
                    port_file.display()
                );
            }
        }

        let running_flag = self.is_running.clone();
        let counter = self.packets_received.clone();
        let last_time = self.last_packet_time.clone();
        let last_sender = self.last_sender.clone();
        let shared = self.shared_state.clone();

        let handle = tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            while running_flag.load(Ordering::SeqCst) {
                match socket.recv_from(&mut buf).await {
                    Ok((len, peer)) => {
                        counter.fetch_add(1, Ordering::SeqCst);
                        *last_time.write() = Some(chrono::Utc::now().timestamp_millis());
                        *last_sender.write() = Some(peer.to_string());

                        if let Ok(text) = std::str::from_utf8(&buf[..len]) {
                            tracing::info!(
                                "DualSenseX UDP packet received from {} ({} bytes)",
                                peer,
                                len
                            );
                            Self::process_payload(text, &shared);
                        }
                    }
                    Err(e) => {
                        if !running_flag.load(Ordering::SeqCst) {
                            break;
                        }
                        tracing::warn!("DualSense UDP socket error: {}", e);
                    }
                }
            }
        });

        self.worker_handle = Some(handle);
        tracing::info!("DualSenseX Link UDP server started on port {}", self.port);
        Ok(())
    }

    pub fn stop(&mut self) {
        if self.is_running.swap(false, Ordering::SeqCst) {
            if let Some(handle) = self.worker_handle.take() {
                handle.abort();
            }
            tracing::info!("DualSenseX Link UDP server stopped");
        }
    }

    pub fn process_payload(payload: &str, shared: &Arc<SharedControllerState>) {
        // Try parsing as DualSenseX packet
        if let Ok(packet) = serde_json::from_str::<DualSenseXPacket>(payload) {
            if let Some(lt) = packet.left_trigger {
                shared.triggers.write().left = lt;
            }
            if let Some(rt) = packet.right_trigger {
                shared.triggers.write().right = rt;
            }
            if let Some(rgb) = packet.rgb {
                *shared.lightbar.write() = rgb;
            }

            // Process instructions array (standard DualSenseX C# protocol)
            for inst in packet.instructions {
                let param_u8 = |idx: usize, default: u8| -> u8 {
                    inst.parameters
                        .get(idx)
                        .and_then(|v| v.as_u64())
                        .unwrap_or(default as u64) as u8
                };

                match inst.inst_type {
                    1 => {
                        // Trigger update: [controller_idx, trigger_idx (1=L2, 2=R2), mode_idx, ...params]
                        if inst.parameters.len() >= 3 {
                            let trigger_idx = inst.parameters[1].as_u64().unwrap_or(1);
                            let mode_idx = inst.parameters[2].as_u64().unwrap_or(0);

                            let effect = match mode_idx {
                                0 => TriggerEffect::Off,
                                1 | 2 => TriggerEffect::Feedback {
                                    start_position: param_u8(3, 0),
                                    force: param_u8(4, 5),
                                },
                                3 => TriggerEffect::Bow {
                                    start_position: param_u8(3, 0),
                                    end_position: param_u8(4, 8),
                                    force: param_u8(5, 4),
                                    snap_force: param_u8(6, 6),
                                },
                                4 => TriggerEffect::Galloping {
                                    start_position: param_u8(3, 0),
                                    end_position: param_u8(4, 8),
                                    first_foot: param_u8(5, 1),
                                    second_foot: param_u8(6, 4),
                                    frequency: param_u8(7, 10),
                                },
                                5 => TriggerEffect::Weapon {
                                    start_position: param_u8(3, 2),
                                    end_position: param_u8(4, 7),
                                    force: param_u8(5, 8),
                                },
                                6 => TriggerEffect::MachineGun {
                                    start_position: param_u8(3, 1),
                                    end_position: param_u8(4, 8),
                                    strength: param_u8(5, 7),
                                    frequency: param_u8(6, 20),
                                },
                                7 => TriggerEffect::Vibration {
                                    position: param_u8(3, 1),
                                    amplitude: param_u8(4, 6),
                                    frequency: param_u8(5, 20),
                                },
                                8 => TriggerEffect::CustomZone {
                                    zones: [
                                        param_u8(3, 0),
                                        param_u8(4, 0),
                                        param_u8(5, 0),
                                        param_u8(6, 0),
                                        param_u8(7, 0),
                                        param_u8(8, 0),
                                        param_u8(9, 0),
                                        param_u8(10, 0),
                                        param_u8(11, 0),
                                        param_u8(12, 0),
                                    ],
                                },
                                9 => TriggerEffect::Machine {
                                    start_position: param_u8(3, 1),
                                    end_position: param_u8(4, 9),
                                    strength_a: param_u8(5, 4),
                                    strength_b: param_u8(6, 7),
                                    frequency: param_u8(7, 15),
                                    period: param_u8(8, 2),
                                },
                                10 => TriggerEffect::Choppy,
                                11 => TriggerEffect::VerySoft,
                                12 => TriggerEffect::Soft,
                                13 => TriggerEffect::Medium,
                                14 => TriggerEffect::Hard,
                                15 => TriggerEffect::VeryHard,
                                16 => TriggerEffect::Hardest,
                                17 => TriggerEffect::Rigid,
                                18 => TriggerEffect::VibrateTriggerPulse,
                                19 => TriggerEffect::GameCube,
                                _ => TriggerEffect::Off,
                            };

                            let mut triggers = shared.triggers.write();
                            if trigger_idx == 1 {
                                triggers.left = effect;
                            } else {
                                triggers.right = effect;
                            }
                        }
                    }
                    2 => {
                        // RGB update: [controller_idx, r, g, b]
                        if inst.parameters.len() >= 4 {
                            let r = param_u8(1, 0);
                            let g = param_u8(2, 0);
                            let b = param_u8(3, 0);
                            *shared.lightbar.write() = RgbColor::new(r, g, b);
                        }
                    }
                    4 => {
                        // Rumble: [controller_idx, heavy, light]
                        if inst.parameters.len() >= 3 {
                            let heavy = param_u8(1, 0);
                            let light = param_u8(2, 0);
                            *shared.heavy_rumble.write() = heavy;
                            *shared.light_rumble.write() = light;
                        }
                    }
                    5 => {
                        // Reset
                        *shared.triggers.write() = DualTriggerConfig::default();
                        *shared.lightbar.write() = RgbColor::SONY_BLUE;
                        *shared.heavy_rumble.write() = 0;
                        *shared.light_rumble.write() = 0;
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Drop for DualSenseUdpServer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_udp_server_lifecycle_and_packet_processing() {
        let shared = Arc::new(SharedControllerState::default());
        let mut server = DualSenseUdpServer::new(16969, shared.clone());

        assert!(!server.stats().running);
        server.start().await.expect("Failed to bind UDP port");
        assert!(server.stats().running);

        // Process a DualSenseX JSON packet directly
        let payload = r#"{
            "instructions": [
                { "type": 1, "parameters": [0, 2, 5, 1, 7, 8] },
                { "type": 2, "parameters": [0, 255, 64, 0] },
                { "type": 4, "parameters": [0, 180, 90] }
            ]
        }"#;

        DualSenseUdpServer::process_payload(payload, &shared);

        // Verify triggers updated (Right trigger is Weapon with 1, 7, 8)
        let triggers = shared.triggers.read().clone();
        assert_eq!(
            triggers.right,
            TriggerEffect::Weapon {
                start_position: 1,
                end_position: 7,
                force: 8,
            }
        );

        // Verify RGB updated
        let rgb = *shared.lightbar.read();
        assert_eq!(rgb, RgbColor::new(255, 64, 0));

        // Verify Rumble updated
        assert_eq!(*shared.heavy_rumble.read(), 180);
        assert_eq!(*shared.light_rumble.read(), 90);

        // Test PascalCase DualSenseX packet & extended modes (Choppy=10, Rigid=17, VibrateTrigger=7, Reset=5)
        let pascal_payload = r#"{
            "Instructions": [
                { "Type": 1, "Parameters": [0, 1, 10] },
                { "Type": 1, "Parameters": [0, 2, 17] }
            ]
        }"#;
        DualSenseUdpServer::process_payload(pascal_payload, &shared);
        assert_eq!(shared.triggers.read().left, TriggerEffect::Choppy);
        assert_eq!(shared.triggers.read().right, TriggerEffect::Rigid);

        let reset_payload = r#"{ "instructions": [{ "type": 5, "parameters": [] }] }"#;
        DualSenseUdpServer::process_payload(reset_payload, &shared);
        assert_eq!(shared.triggers.read().left, TriggerEffect::Off);

        server.stop();
        assert!(!server.stats().running);
    }
}
