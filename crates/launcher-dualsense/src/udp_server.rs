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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualSenseXInstruction {
    #[serde(rename = "type")]
    pub inst_type: u32,
    pub parameters: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualSenseXPacket {
    #[serde(default)]
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
                match inst.inst_type {
                    1 => {
                        // Trigger update: [controller_idx, trigger_idx (1=L2, 2=R2), mode_idx, ...params]
                        if inst.parameters.len() >= 3 {
                            let trigger_idx = inst.parameters[1].as_u64().unwrap_or(1);
                            let mode_idx = inst.parameters[2].as_u64().unwrap_or(0);

                            let effect = match mode_idx {
                                0 => TriggerEffect::Off,
                                1 | 2 => {
                                    let p1 = inst
                                        .parameters
                                        .get(3)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0)
                                        as u8;
                                    let p2 = inst
                                        .parameters
                                        .get(4)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(5)
                                        as u8;
                                    TriggerEffect::Feedback {
                                        start_position: p1,
                                        force: p2,
                                    }
                                }
                                3 => {
                                    let p1 = inst
                                        .parameters
                                        .get(3)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0)
                                        as u8;
                                    let p2 = inst
                                        .parameters
                                        .get(4)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(8)
                                        as u8;
                                    let p3 = inst
                                        .parameters
                                        .get(5)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(4)
                                        as u8;
                                    let p4 = inst
                                        .parameters
                                        .get(6)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(6)
                                        as u8;
                                    TriggerEffect::Bow {
                                        start_position: p1,
                                        end_position: p2,
                                        force: p3,
                                        snap_force: p4,
                                    }
                                }
                                4 => {
                                    let p1 = inst
                                        .parameters
                                        .get(3)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0)
                                        as u8;
                                    let p2 = inst
                                        .parameters
                                        .get(4)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(8)
                                        as u8;
                                    let p3 = inst
                                        .parameters
                                        .get(5)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(1)
                                        as u8;
                                    let p4 = inst
                                        .parameters
                                        .get(6)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(4)
                                        as u8;
                                    let p5 = inst
                                        .parameters
                                        .get(7)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(10)
                                        as u8;
                                    TriggerEffect::Galloping {
                                        start_position: p1,
                                        end_position: p2,
                                        first_foot: p3,
                                        second_foot: p4,
                                        frequency: p5,
                                    }
                                }
                                5 => {
                                    let p1 = inst
                                        .parameters
                                        .get(3)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(2)
                                        as u8;
                                    let p2 = inst
                                        .parameters
                                        .get(4)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(7)
                                        as u8;
                                    let p3 = inst
                                        .parameters
                                        .get(5)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(8)
                                        as u8;
                                    TriggerEffect::Weapon {
                                        start_position: p1,
                                        end_position: p2,
                                        force: p3,
                                    }
                                }
                                6 => {
                                    let p1 = inst
                                        .parameters
                                        .get(3)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(1)
                                        as u8;
                                    let p2 = inst
                                        .parameters
                                        .get(4)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(8)
                                        as u8;
                                    let p3 = inst
                                        .parameters
                                        .get(5)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(7)
                                        as u8;
                                    let p4 = inst
                                        .parameters
                                        .get(6)
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(20)
                                        as u8;
                                    TriggerEffect::MachineGun {
                                        start_position: p1,
                                        end_position: p2,
                                        strength: p3,
                                        frequency: p4,
                                    }
                                }
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
                            let r = inst.parameters[1].as_u64().unwrap_or(0) as u8;
                            let g = inst.parameters[2].as_u64().unwrap_or(0) as u8;
                            let b = inst.parameters[3].as_u64().unwrap_or(0) as u8;
                            *shared.lightbar.write() = RgbColor::new(r, g, b);
                        }
                    }
                    4 => {
                        // Rumble: [controller_idx, heavy, light]
                        if inst.parameters.len() >= 3 {
                            let heavy = inst.parameters[1].as_u64().unwrap_or(0) as u8;
                            let light = inst.parameters[2].as_u64().unwrap_or(0) as u8;
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

        server.stop();
        assert!(!server.stats().running);
    }
}
