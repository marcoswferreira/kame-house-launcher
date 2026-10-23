use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::manager::DualSenseManager;
use crate::protocol::{ConnectionType, OutputReportBuilder};

pub const SONY_VENDOR_ID: u16 = 0x054c;
pub const DUALSENSE_PID: u16 = 0x0ce6;
pub const DUALSENSE_EDGE_PID: u16 = 0x0df2;
pub const DUALSHOCK4_V1_PID: u16 = 0x05c4;
pub const DUALSHOCK4_V2_PID: u16 = 0x09cc;

/// Checks if a USB/Bluetooth HID device matches supported Sony controllers
pub fn is_supported_sony_controller(vendor_id: u16, product_id: u16) -> bool {
    vendor_id == SONY_VENDOR_ID
        && (product_id == DUALSENSE_PID
            || product_id == DUALSENSE_EDGE_PID
            || product_id == DUALSHOCK4_V1_PID
            || product_id == DUALSHOCK4_V2_PID)
}

/// Identifies controller name and whether it is a DualSense (with adaptive triggers)
pub fn identify_controller(product_id: u16) -> (&'static str, bool) {
    match product_id {
        DUALSENSE_PID => ("Sony DualSense Wireless Controller", true),
        DUALSENSE_EDGE_PID => ("Sony DualSense Edge Wireless Controller", true),
        p if p == DUALSHOCK4_V1_PID || p == DUALSHOCK4_V2_PID => {
            ("Sony DualShock 4 Wireless Controller", false)
        }
        _ => ("Sony Gamepad", false),
    }
}

pub struct HidDeviceWorker {
    running: Arc<AtomicBool>,
}

impl HidDeviceWorker {
    pub fn start(manager: Arc<DualSenseManager>) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let worker_running = running.clone();

        thread::Builder::new()
            .name("dualsense-hid-worker".to_string())
            .spawn(move || {
                info!("Starting DualSense/DualShock HID hardware worker");

                while worker_running.load(Ordering::Relaxed) {
                    let api = match hidapi::HidApi::new() {
                        Ok(api) => api,
                        Err(e) => {
                            debug!("HidApi init waiting: {}. Retrying in 2s...", e);
                            thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                    };

                    let mut target_device = None;
                    let mut is_ds = true;
                    let mut controller_name = "Sony DualSense Wireless Controller";

                    for dev_info in api.device_list() {
                        if is_supported_sony_controller(
                            dev_info.vendor_id(),
                            dev_info.product_id(),
                        ) {
                            let (name, is_dualsense) =
                                identify_controller(dev_info.product_id());
                            controller_name = name;
                            is_ds = is_dualsense;
                            target_device = Some(dev_info.clone());
                            break;
                        }
                    }

                    let dev_info = match target_device {
                        Some(d) => d,
                        None => {
                            // No physical device visible directly in this OS environment (e.g. WSL without usbipd)
                            thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                    };

                    info!(
                        "Discovered physical Sony controller: {} ({:04x}:{:04x})",
                        controller_name,
                        dev_info.vendor_id(),
                        dev_info.product_id()
                    );

                    let device = match dev_info.open_device(&api) {
                        Ok(dev) => dev,
                        Err(e) => {
                            warn!("Failed to open HID device: {}. Retrying in 2s...", e);
                            thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                    };

                    manager.set_connected(true, controller_name);
                    let connection = ConnectionType::Usb;

                    let mut in_buf = [0u8; 128];
                    let mut last_send = std::time::Instant::now();

                    while worker_running.load(Ordering::Relaxed) {
                        // Read incoming input report (timeout 10ms for 100Hz responsiveness)
                        match device.read_timeout(&mut in_buf, 10) {
                            Ok(bytes_read) if bytes_read > 0 => {
                                let slice = &in_buf[..bytes_read];
                                let parsed = if slice[0] == 0x01 && bytes_read >= 10 {
                                    crate::protocol::parse_input_report(slice, ConnectionType::Usb)
                                } else if slice[0] == 0x31 && bytes_read >= 12 {
                                    crate::protocol::parse_input_report(slice, ConnectionType::Bluetooth)
                                } else if bytes_read >= 9 {
                                    crate::protocol::parse_input_report(slice, connection)
                                } else {
                                    None
                                };

                                if let Some(state) = parsed {
                                    if let Some(muted) = manager.update_input_state(state) {
                                        info!("Physical DualSense Mic Mute toggled: muted={}", muted);
                                    }
                                }
                            }
                            Ok(_) => {
                                // Timeout expired with no report, proceed to check output send
                            }
                            Err(e) => {
                                warn!("HID device read error: {}. Re-enumerating...", e);
                                break;
                            }
                        }

                        // Send Output Report (Adaptive Triggers, Lightbar, Player LEDs) periodically (40Hz)
                        if is_ds && last_send.elapsed() >= Duration::from_millis(25) {
                            last_send = std::time::Instant::now();
                            let builder = manager.build_current_output_report();

                            let report = match connection {
                                ConnectionType::Usb => builder.build_usb().to_vec(),
                                ConnectionType::Bluetooth => builder.build_bluetooth().to_vec(),
                            };

                            if let Err(e) = device.write(&report) {
                                debug!("Failed to write output report to controller: {}", e);
                            }
                        }
                    }

                    manager.set_connected(false, "Disconnected");
                    info!("Controller disconnected or worker resetting.");
                    thread::sleep(Duration::from_secs(1));
                }
            })
            .expect("Failed to spawn dualsense-hid-worker");

        Self { running }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Drop for HidDeviceWorker {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_controller_identification() {
        assert!(is_supported_sony_controller(SONY_VENDOR_ID, DUALSENSE_PID));
        assert!(is_supported_sony_controller(SONY_VENDOR_ID, DUALSENSE_EDGE_PID));
        assert!(is_supported_sony_controller(SONY_VENDOR_ID, DUALSHOCK4_V1_PID));
        assert!(is_supported_sony_controller(SONY_VENDOR_ID, DUALSHOCK4_V2_PID));
        assert!(!is_supported_sony_controller(0x045e, 0x028e)); // Xbox 360

        let (ds_name, is_ds) = identify_controller(DUALSENSE_PID);
        assert!(is_ds);
        assert!(ds_name.contains("DualSense"));

        let (ds4_name, is_ds4) = identify_controller(DUALSOCK4_V1_PID);
        assert!(!is_ds4);
        assert!(ds4_name.contains("DualShock 4"));
    }
}
