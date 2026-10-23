use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::manager::DualSenseManager;
use crate::protocol::ConnectionType;

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

/// Detects initial connection transport (USB vs Bluetooth) from OS HID path and interface number
pub fn detect_connection_type(path: &str, interface_number: i32) -> ConnectionType {
    let lower = path.to_ascii_lowercase();
    if lower.contains("bth")
        || lower.contains("{00001124-")
        || lower.contains("bluetooth")
        || (interface_number == -1 && !lower.contains("usb") && !lower.is_empty())
    {
        ConnectionType::Bluetooth
    } else {
        ConnectionType::Usb
    }
}

/// Dynamically refines connection transport from incoming HID input report header and packet size
pub fn detect_connection_from_input_report(slice: &[u8]) -> Option<ConnectionType> {
    if slice.is_empty() {
        return None;
    }
    if slice[0] == 0x31 || slice.len() == 78 || (slice[0] == 0x01 && slice.len() <= 12) {
        Some(ConnectionType::Bluetooth)
    } else if slice[0] == 0x01 && slice.len() >= 60 {
        Some(ConnectionType::Usb)
    } else {
        None
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
                    if !manager.is_extension_enabled() {
                        manager.set_connected(false, "Disconnected");
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }

                    let api = match hidapi::HidApi::new() {
                        Ok(api) => api,
                        Err(e) => {
                            debug!("HidApi init waiting: {}. Retrying in 2s...", e);
                            thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                    };

                    let mut candidates: Vec<_> = api
                        .device_list()
                        .filter(|d| is_supported_sony_controller(d.vendor_id(), d.product_id()))
                        .cloned()
                        .collect();

                    if candidates.is_empty() {
                        manager.set_connected(false, "Disconnected");
                        // No physical device visible directly in this OS environment
                        thread::sleep(Duration::from_secs(2));
                        continue;
                    }

                    // Prioritize Gamepad / Joystick interfaces:
                    // 1. Generic Desktop Gamepad (usage_page = 0x01, usage = 0x05) or Joystick (0x04)
                    // 2. Generic Desktop Page (usage_page = 0x01)
                    // 3. Fallback / unspecified (usage_page = 0)
                    // 4. Other collections (audio, touchpad, vendor)
                    candidates.sort_by_key(|d| {
                        let up = d.usage_page();
                        let u = d.usage();
                        if up == 0x01 && (u == 0x05 || u == 0x04) {
                            0
                        } else if up == 0x01 {
                            1
                        } else if up == 0 {
                            2
                        } else {
                            3
                        }
                    });

                    let dev_info = &candidates[0];
                    let (name, is_dualsense) = identify_controller(dev_info.product_id());
                    let controller_name = name;
                    let is_ds = is_dualsense;

                    let path_str = dev_info.path().to_string_lossy().to_string();
                    let mut connection =
                        detect_connection_type(&path_str, dev_info.interface_number());

                    info!(
                        "Discovered physical Sony controller: {} ({:04x}:{:04x}) | path='{}' | interface={} | usage_page=0x{:04x} | usage=0x{:04x} | initial_transport={:?}",
                        controller_name,
                        dev_info.vendor_id(),
                        dev_info.product_id(),
                        path_str,
                        dev_info.interface_number(),
                        dev_info.usage_page(),
                        dev_info.usage(),
                        connection
                    );

                    let device = match dev_info.open_device(&api) {
                        Ok(dev) => dev,
                        Err(e) => {
                            warn!("Failed to open HID device ({}): {}. Retrying in 2s...", path_str, e);
                            thread::sleep(Duration::from_secs(2));
                            continue;
                        }
                    };

                    // Query Feature Report 0x05 (Calibration data) to unlock DualSense Bluetooth Enhanced Mode (0x31)
                    if is_ds {
                        let mut cal_report = [0u8; 41];
                        cal_report[0] = 0x05;
                        match device.get_feature_report(&mut cal_report) {
                            Ok(len) => {
                                info!(
                                    "DualSense Feature Report 0x05 read succeeded ({} bytes) — enhanced mode unlocked",
                                    len
                                );
                            }
                            Err(e) => {
                                warn!(
                                    "DualSense Feature Report 0x05 query returned error (controller may already be in active mode): {}",
                                    e
                                );
                            }
                        }
                    }

                    let mut in_buf = [0u8; 128];
                    let mut last_send = std::time::Instant::now();
                    let mut last_input_time = std::time::Instant::now();
                    let mut is_currently_connected = false;
                    let mut bt_seq: u8 = 0;
                    let mut logged_first_input = false;
                    let mut last_logged_state: Option<String> = None;
                    let mut write_error_count: u32 = 0;
                    let mut read_error_count: u32 = 0;

                    while worker_running.load(Ordering::Relaxed) {
                        if !manager.is_extension_enabled() {
                            is_currently_connected = false;
                            manager.set_connected(false, "Disconnected");
                            break;
                        }

                        // Read incoming input report (timeout 10ms for 100Hz responsiveness)
                        match device.read_timeout(&mut in_buf, 10) {
                            Ok(bytes_read) if bytes_read > 0 => {
                                read_error_count = 0;
                                let slice = &in_buf[..bytes_read];
                                last_input_time = std::time::Instant::now();

                                if !is_currently_connected {
                                    is_currently_connected = true;
                                    manager.set_connected(true, controller_name);
                                    info!(
                                        "DualSense/Sony controller connection verified and active: {}",
                                        controller_name
                                    );
                                }

                                if let Some(detected) = detect_connection_from_input_report(slice) {
                                    if detected != connection {
                                        info!(
                                            "DualSense transport updated from {:?} -> {:?} based on input report (report_id=0x{:02x}, len={}B)",
                                            connection, detected, slice[0], bytes_read
                                        );
                                        connection = detected;
                                    }
                                }

                                if !logged_first_input {
                                    info!(
                                        "First DualSense HID input report received: report_id=0x{:02x}, len={} bytes, active_transport={:?}",
                                        slice[0], bytes_read, connection
                                    );
                                    logged_first_input = true;
                                }

                                let parsed = crate::protocol::parse_input_report(slice, connection);

                                if let Some(state) = parsed {
                                    if let Some(muted) = manager.update_input_state(state) {
                                        info!(
                                            "Physical DualSense Mic Mute toggled: muted={}",
                                            muted
                                        );
                                    }
                                }
                            }
                            Ok(_) => {
                                // Timeout expired with no report read
                                if is_currently_connected && last_input_time.elapsed() > Duration::from_millis(3000) {
                                    warn!("DualSense input report timeout (>3s). Controller disconnected or turned off.");
                                    is_currently_connected = false;
                                    manager.set_connected(false, "Disconnected");
                                    break;
                                } else if !is_currently_connected && last_input_time.elapsed() > Duration::from_millis(3000) {
                                    debug!("Device opened but did not produce any input reports in 3s. Closing device.");
                                    break;
                                }
                            }
                            Err(e) => {
                                read_error_count += 1;
                                if read_error_count >= 3 {
                                    warn!("HID device consecutive read errors ({}): {}. Disconnecting controller.", read_error_count, e);
                                    is_currently_connected = false;
                                    manager.set_connected(false, "Disconnected");
                                    break;
                                }
                            }
                        }

                        // Send Output Report (Adaptive Triggers, Lightbar, Player LEDs) periodically (40Hz)
                        if is_ds && last_send.elapsed() >= Duration::from_millis(25) {
                            last_send = std::time::Instant::now();
                            let current_seq = bt_seq;
                            let builder = manager
                                .build_current_output_report()
                                .with_seq(current_seq);

                            let report = match connection {
                                ConnectionType::Usb => builder.build_usb().to_vec(),
                                ConnectionType::Bluetooth => {
                                    bt_seq = (bt_seq + 1) & 0x0F;
                                    builder.build_bluetooth().to_vec()
                                }
                            };

                            let state_summary = format!(
                                "{:?}|RGB({},{},{})|Player({:?}:0x{:02x})|Mic({:?})|L2({})|R2({})|Rumble({},{})",
                                connection,
                                builder.lightbar_color.r,
                                builder.lightbar_color.g,
                                builder.lightbar_color.b,
                                builder.player_led,
                                builder.player_led.to_bitmask(),
                                builder.mic_led,
                                builder.triggers.left.name(),
                                builder.triggers.right.name(),
                                builder.heavy_rumble,
                                builder.light_rumble
                            );

                            let state_changed =
                                last_logged_state.as_deref() != Some(state_summary.as_str());

                            if state_changed {
                                info!(
                                    "Dispatching DualSense output report [id=0x{:02x}, len={}B, seq={}]: {}",
                                    report[0],
                                    report.len(),
                                    current_seq,
                                    state_summary
                                );
                            }

                            match device.write(&report) {
                                Ok(written) => {
                                    if state_changed {
                                        info!(
                                            "DualSense HID write succeeded: {} bytes written (transport={:?}, valid_flag1=0x{:02x})",
                                            written,
                                            connection,
                                            if connection == ConnectionType::Usb {
                                                report[2]
                                            } else {
                                                report[4]
                                            }
                                        );
                                        last_logged_state = Some(state_summary);
                                    }
                                    write_error_count = 0;
                                }
                                Err(e) => {
                                    write_error_count += 1;
                                    if write_error_count <= 3 || write_error_count.is_multiple_of(100) {
                                        warn!(
                                            "Failed to write {:?} output report (id=0x{:02x}, len={}B) to DualSense: {} (consecutive_errors={}). Pausing output writes temporarily.",
                                            connection,
                                            report[0],
                                            report.len(),
                                            e,
                                            write_error_count
                                        );
                                    }
                                    // Never disconnect the controller on write error! Input polling and gamepad emulation continue running.
                                    last_send = std::time::Instant::now() + Duration::from_secs(2);
                                }
                            }
                        }
                    }

                    if is_currently_connected {
                        manager.set_connected(false, "Disconnected");
                    }
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
        assert!(is_supported_sony_controller(
            SONY_VENDOR_ID,
            DUALSENSE_EDGE_PID
        ));
        assert!(is_supported_sony_controller(
            SONY_VENDOR_ID,
            DUALSHOCK4_V1_PID
        ));
        assert!(is_supported_sony_controller(
            SONY_VENDOR_ID,
            DUALSHOCK4_V2_PID
        ));
        assert!(!is_supported_sony_controller(0x045e, 0x028e)); // Xbox 360

        let (ds_name, is_ds) = identify_controller(DUALSENSE_PID);
        assert!(is_ds);
        assert!(ds_name.contains("DualSense"));

        let (ds4_name, is_ds4) = identify_controller(DUALSHOCK4_V1_PID);
        assert!(!is_ds4);
        assert!(ds4_name.contains("DualShock 4"));
    }

    #[test]
    fn test_detect_connection_type_and_input_reports() {
        assert_eq!(
            detect_connection_type(
                r"\\?\hid#{00001124-0000-1000-8000-00805f9b34fb}_vid&0002054c_pid&0ce6",
                -1
            ),
            ConnectionType::Bluetooth
        );
        assert_eq!(
            detect_connection_type(r"\\?\hid#vid_054c&pid_0ce6&mi_03#7", 3),
            ConnectionType::Usb
        );
        assert_eq!(
            detect_connection_type("/dev/hidraw0", -1),
            ConnectionType::Bluetooth
        );
        assert_eq!(
            detect_connection_type("/dev/hidraw0", 3),
            ConnectionType::Usb
        );

        assert_eq!(detect_connection_from_input_report(&[]), None);
        let mut bt_rep = [0u8; 78];
        bt_rep[0] = 0x31;
        assert_eq!(
            detect_connection_from_input_report(&bt_rep),
            Some(ConnectionType::Bluetooth)
        );
        let mut bt_short = [0u8; 10];
        bt_short[0] = 0x01;
        assert_eq!(
            detect_connection_from_input_report(&bt_short),
            Some(ConnectionType::Bluetooth)
        );
        let mut usb_rep = [0u8; 64];
        usb_rep[0] = 0x01;
        assert_eq!(
            detect_connection_from_input_report(&usb_rep),
            Some(ConnectionType::Usb)
        );
    }
}
