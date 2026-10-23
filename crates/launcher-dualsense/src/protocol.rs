use byteorder::{ByteOrder, LittleEndian};
use crc32fast::Hasher;
use serde::{Deserialize, Serialize};

use crate::lightbar::{MicLed, PlayerLed, RgbColor};
use crate::triggers::DualTriggerConfig;

pub const SONY_VENDOR_ID: u16 = 0x054C;
pub const DUALSENSE_PID: u16 = 0x0CE6;
pub const DUALSENSE_EDGE_PID: u16 = 0x0DF2;

pub const USB_OUTPUT_REPORT_SIZE: usize = 64;
pub const BT_OUTPUT_REPORT_SIZE: usize = 78;

/// DualSense connection transport
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionType {
    #[default]
    Usb,
    Bluetooth,
}

/// Controller Battery state
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BatteryState {
    pub percentage: u8,
    pub is_charging: bool,
    pub is_full: bool,
}

/// 6-Axis Motion Telemetry (Gyroscope + Accelerometer)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MotionData {
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
}

/// DualSense Touchpad Contact Point
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TouchPoint {
    pub active: bool,
    pub id: u8,
    pub x: u16,
    pub y: u16,
}

/// High-level parsed state of DualSense inputs
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DualSenseInputState {
    pub connection: ConnectionType,
    // Analog Sticks (0..255, 128 is center)
    pub left_stick_x: u8,
    pub left_stick_y: u8,
    pub right_stick_x: u8,
    pub right_stick_y: u8,
    // Analog Triggers (0..255)
    pub l2_trigger: u8,
    pub r2_trigger: u8,
    // Action Buttons
    pub cross: bool,
    pub circle: bool,
    pub square: bool,
    pub triangle: bool,
    // D-Pad
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
    // Bumpers & Clicks
    pub l1: bool,
    pub r1: bool,
    pub l2_click: bool,
    pub r2_click: bool,
    pub l3: bool,
    pub r3: bool,
    // System Buttons
    pub create: bool,
    pub options: bool,
    pub ps_button: bool,
    pub touchpad_click: bool,
    pub mic_mute: bool,
    // Telemetry
    pub battery: BatteryState,
    pub motion: MotionData,
    pub touch1: TouchPoint,
    pub touch2: TouchPoint,
}

/// Builder for DualSense Output Reports (USB 0x02 and Bluetooth 0x31)
#[derive(Debug, Clone)]
pub struct OutputReportBuilder {
    pub triggers: DualTriggerConfig,
    pub lightbar_color: RgbColor,
    pub player_led: PlayerLed,
    pub mic_led: MicLed,
    pub heavy_rumble: u8,
    pub light_rumble: u8,
    pub seq_num: u8,
}

impl Default for OutputReportBuilder {
    fn default() -> Self {
        Self {
            triggers: DualTriggerConfig::default(),
            lightbar_color: RgbColor::SONY_BLUE,
            player_led: PlayerLed::Player1,
            mic_led: MicLed::Off,
            heavy_rumble: 0,
            light_rumble: 0,
            seq_num: 0,
        }
    }
}

impl OutputReportBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_triggers(mut self, triggers: DualTriggerConfig) -> Self {
        self.triggers = triggers;
        self
    }

    pub fn with_lightbar(mut self, color: RgbColor) -> Self {
        self.lightbar_color = color;
        self
    }

    pub fn with_player_led(mut self, led: PlayerLed) -> Self {
        self.player_led = led;
        self
    }

    pub fn with_mic_led(mut self, led: MicLed) -> Self {
        self.mic_led = led;
        self
    }

    pub fn with_rumble(mut self, heavy: u8, light: u8) -> Self {
        self.heavy_rumble = heavy;
        self.light_rumble = light;
        self
    }

    pub fn with_seq(mut self, seq_num: u8) -> Self {
        self.seq_num = seq_num;
        self
    }

    /// Builds a 64-byte USB Output Report (Report ID 0x02)
    pub fn build_usb(&self) -> [u8; USB_OUTPUT_REPORT_SIZE] {
        let mut report = [0u8; USB_OUTPUT_REPORT_SIZE];
        report[0] = 0x02; // Report ID

        // valid_flag0: Enable compatible rumble (0x01 | 0x02), triggers (0x04 | 0x08)
        report[1] = 0xFF;

        // valid_flag1: Enable mic mute led (0x01), lightbar (0x04), player leds (0x10).
        // IMPORTANT: Bit 0x08 (DS_OUTPUT_VALID_FLAG1_RELEASE_LEDS) MUST NOT be set,
        // otherwise the controller firmware resets and turns off all LEDs.
        report[2] = 0x15;

        // Rumble motors
        report[3] = self.light_rumble;
        report[4] = self.heavy_rumble;

        // Mic LED (0=off, 1=solid amber, 2=pulse)
        report[9] = self.mic_led.to_byte();

        // Right Trigger effect (11 bytes starting at index 11)
        let rt = self.triggers.right.compile_payload();
        report[11..22].copy_from_slice(&rt);

        // Left Trigger effect (11 bytes starting at index 22)
        let lt = self.triggers.left.compile_payload();
        report[22..33].copy_from_slice(&lt);

        // valid_flag2: Enable lightbar setup (DS_OUTPUT_VALID_FLAG2_LIGHTBAR_SETUP_CONTROL_ENABLE)
        report[39] = 0x02;

        // lightbar_setup: 0x02 = DS_OUTPUT_LIGHTBAR_SETUP_LIGHT_OUT (clears default fade-in so RGB takes over)
        report[42] = 0x02;

        // led_brightness: 0x00 = High brightness
        report[43] = 0x00;

        // Player LEDs
        report[44] = self.player_led.to_bitmask();

        // Lightbar RGB
        report[45] = self.lightbar_color.r;
        report[46] = self.lightbar_color.g;
        report[47] = self.lightbar_color.b;

        report
    }

    /// Builds a 78-byte Bluetooth Output Report (Report ID 0x31) with CRC32
    pub fn build_bluetooth(&self) -> [u8; BT_OUTPUT_REPORT_SIZE] {
        let mut report = [0u8; BT_OUTPUT_REPORT_SIZE];
        report[0] = 0x31; // Report ID
                          // Highest 4-bit is rolling sequence number (0..15), lowest 4-bit is 0x02 flag
        report[1] = ((self.seq_num & 0x0F) << 4) | 0x02;
        report[2] = 0x10; // DS_OUTPUT_TAG (0x10)

        // valid_flag0: Enable rumble and triggers (offset 3)
        report[3] = 0xFF;

        // valid_flag1: Enable mic led (0x01), lightbar (0x04), player leds (0x10) (offset 4)
        // Must NOT set 0x08 (RELEASE_LEDS)
        report[4] = 0x15;

        // Rumble
        report[5] = self.light_rumble;
        report[6] = self.heavy_rumble;

        // Mic LED (common[8] is at index 3 + 8 = 11)
        report[11] = self.mic_led.to_byte();

        // Right Trigger effect (common[10] is at index 3 + 10 = 13)
        let rt = self.triggers.right.compile_payload();
        report[13..24].copy_from_slice(&rt);

        // Left Trigger effect (common[21] is at index 3 + 21 = 24)
        let lt = self.triggers.left.compile_payload();
        report[24..35].copy_from_slice(&lt);

        // valid_flag2: Enable lightbar setup (common[38] is at index 3 + 38 = 41)
        report[41] = 0x02;

        // lightbar_setup: 0x02 = DS_OUTPUT_LIGHTBAR_SETUP_LIGHT_OUT (common[41] is at index 3 + 41 = 44)
        report[44] = 0x02;

        // led_brightness: 0x00 = High brightness (common[42] is at index 3 + 42 = 45)
        report[45] = 0x00;

        // Player LEDs (common[43] is at index 3 + 43 = 46)
        report[46] = self.player_led.to_bitmask();

        // Lightbar RGB (common[44..47] is at index 3 + 44 = 47..50)
        report[47] = self.lightbar_color.r;
        report[48] = self.lightbar_color.g;
        report[49] = self.lightbar_color.b;

        // Calculate CRC32 for Sony BT HID
        let mut hasher = Hasher::new();
        hasher.update(&[0xA2]);
        hasher.update(&report[0..74]);
        let checksum = hasher.finalize();

        LittleEndian::write_u32(&mut report[74..78], checksum);
        report
    }
}

/// Parses raw HID input reports from USB (0x01) or Bluetooth (0x01 / 0x31)
pub fn parse_input_report(buf: &[u8], connection: ConnectionType) -> Option<DualSenseInputState> {
    if buf.is_empty() {
        return None;
    }

    // Bluetooth Compact Report (Report ID 0x01, standard ~10-byte report over Bluetooth)
    if connection == ConnectionType::Bluetooth && buf[0] == 0x01 && buf.len() < 40 {
        if buf.len() < 7 {
            return None;
        }

        let lx = buf[1];
        let ly = buf[2];
        let rx = buf[3];
        let ry = buf[4];

        let buttons1 = buf[5];
        let buttons2 = if buf.len() > 6 { buf[6] } else { 0 };
        let buttons3 = if buf.len() > 7 { buf[7] } else { 0 };

        let dpad_val = buttons1 & 0x0F;
        let (dpad_up, dpad_right, dpad_down, dpad_left) = match dpad_val {
            0 => (true, false, false, false),
            1 => (true, true, false, false),
            2 => (false, true, false, false),
            3 => (false, true, true, false),
            4 => (false, false, true, false),
            5 => (false, false, true, true),
            6 => (false, false, false, true),
            7 => (true, false, false, true),
            _ => (false, false, false, false),
        };

        let square = (buttons1 & 0x10) != 0;
        let cross = (buttons1 & 0x20) != 0;
        let circle = (buttons1 & 0x40) != 0;
        let triangle = (buttons1 & 0x80) != 0;

        let l1 = (buttons2 & 0x01) != 0;
        let r1 = (buttons2 & 0x02) != 0;
        let l2_click = (buttons2 & 0x04) != 0;
        let r2_click = (buttons2 & 0x08) != 0;
        let create = (buttons2 & 0x10) != 0;
        let options = (buttons2 & 0x20) != 0;
        let l3 = (buttons2 & 0x40) != 0;
        let r3 = (buttons2 & 0x80) != 0;

        let ps_button = (buttons3 & 0x01) != 0;
        let touchpad_click = (buttons3 & 0x02) != 0;
        let mic_mute = (buttons3 & 0x04) != 0;

        let l2_trigger = if buf.len() > 8 {
            buf[8]
        } else if l2_click {
            255
        } else {
            0
        };

        let r2_trigger = if buf.len() > 9 {
            buf[9]
        } else if r2_click {
            255
        } else {
            0
        };

        return Some(DualSenseInputState {
            connection,
            left_stick_x: lx,
            left_stick_y: ly,
            right_stick_x: rx,
            right_stick_y: ry,
            l2_trigger,
            r2_trigger,
            cross,
            circle,
            square,
            triangle,
            dpad_up,
            dpad_down,
            dpad_left,
            dpad_right,
            l1,
            r1,
            l2_click,
            r2_click,
            l3,
            r3,
            create,
            options,
            ps_button,
            touchpad_click,
            mic_mute,
            battery: BatteryState {
                percentage: 100,
                is_charging: false,
                is_full: true,
            },
            ..Default::default()
        });
    }

    if buf.len() < 10 {
        return None;
    }

    let (offset, has_telemetry) = match connection {
        ConnectionType::Usb => (1, buf.len() >= 40),
        ConnectionType::Bluetooth => {
            if buf[0] == 0x31 && buf.len() >= 42 {
                (2, true)
            } else {
                (1, buf.len() >= 40)
            }
        }
    };

    if buf.len() < offset + 9 {
        return None;
    }

    let lx = buf[offset];
    let ly = buf[offset + 1];
    let rx = buf[offset + 2];
    let ry = buf[offset + 3];
    let l2 = buf[offset + 4];
    let r2 = buf[offset + 5];

    let buttons1 = buf[offset + 7];
    let buttons2 = buf[offset + 8];
    let buttons3 = if buf.len() > offset + 9 {
        buf[offset + 9]
    } else {
        0
    };

    let dpad_val = buttons1 & 0x0F;
    let (dpad_up, dpad_right, dpad_down, dpad_left) = match dpad_val {
        0 => (true, false, false, false),
        1 => (true, true, false, false),
        2 => (false, true, false, false),
        3 => (false, true, true, false),
        4 => (false, false, true, false),
        5 => (false, false, true, true),
        6 => (false, false, false, true),
        7 => (true, false, false, true),
        _ => (false, false, false, false),
    };

    let square = (buttons1 & 0x10) != 0;
    let cross = (buttons1 & 0x20) != 0;
    let circle = (buttons1 & 0x40) != 0;
    let triangle = (buttons1 & 0x80) != 0;

    let l1 = (buttons2 & 0x01) != 0;
    let r1 = (buttons2 & 0x02) != 0;
    let l2_click = (buttons2 & 0x04) != 0;
    let r2_click = (buttons2 & 0x08) != 0;
    let create = (buttons2 & 0x10) != 0;
    let options = (buttons2 & 0x20) != 0;
    let l3 = (buttons2 & 0x40) != 0;
    let r3 = (buttons2 & 0x80) != 0;

    let ps_button = (buttons3 & 0x01) != 0;
    let touchpad_click = (buttons3 & 0x02) != 0;
    let mic_mute = (buttons3 & 0x04) != 0;

    let mut state = DualSenseInputState {
        connection,
        left_stick_x: lx,
        left_stick_y: ly,
        right_stick_x: rx,
        right_stick_y: ry,
        l2_trigger: l2,
        r2_trigger: r2,
        cross,
        circle,
        square,
        triangle,
        dpad_up,
        dpad_down,
        dpad_left,
        dpad_right,
        l1,
        r1,
        l2_click,
        r2_click,
        l3,
        r3,
        create,
        options,
        ps_button,
        touchpad_click,
        mic_mute,
        ..Default::default()
    };

    if has_telemetry && buf.len() >= offset + 40 {
        state.motion.gyro_x = LittleEndian::read_i16(&buf[offset + 15..offset + 17]);
        state.motion.gyro_y = LittleEndian::read_i16(&buf[offset + 17..offset + 19]);
        state.motion.gyro_z = LittleEndian::read_i16(&buf[offset + 19..offset + 21]);
        state.motion.accel_x = LittleEndian::read_i16(&buf[offset + 21..offset + 23]);
        state.motion.accel_y = LittleEndian::read_i16(&buf[offset + 23..offset + 25]);
        state.motion.accel_z = LittleEndian::read_i16(&buf[offset + 25..offset + 27]);

        let battery_idx = offset + 52;
        if buf.len() > battery_idx {
            let bat_byte = buf[battery_idx];
            let raw_level = bat_byte & 0x0F;
            let status = (bat_byte & 0xF0) >> 4;
            state.battery.percentage = (raw_level.min(10) * 10).min(100);
            state.battery.is_charging = status == 1 || status == 2;
            state.battery.is_full = raw_level >= 10 && !state.battery.is_charging;
        } else {
            state.battery.percentage = 100;
        }

        let touch_idx = offset + 32;
        if buf.len() >= touch_idx + 4 {
            let b0 = buf[touch_idx];
            let b1 = buf[touch_idx + 1];
            let b2 = buf[touch_idx + 2];
            let b3 = buf[touch_idx + 3];

            state.touch1.active = (b0 & 0x80) == 0;
            state.touch1.id = b0 & 0x7F;
            state.touch1.x = (b1 as u16) | (((b2 & 0x0F) as u16) << 8);
            state.touch1.y = ((b2 >> 4) as u16) | ((b3 as u16) << 4);
        }
    } else {
        state.battery.percentage = 100;
    }

    Some(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::triggers::TriggerEffect;

    #[test]
    fn test_usb_report_structure() {
        let builder = OutputReportBuilder::new()
            .with_lightbar(RgbColor::RED)
            .with_player_led(PlayerLed::Player2)
            .with_mic_led(MicLed::Solid)
            .with_triggers(DualTriggerConfig {
                left: TriggerEffect::Feedback {
                    start_position: 1,
                    force: 5,
                },
                right: TriggerEffect::Weapon {
                    start_position: 2,
                    end_position: 6,
                    force: 7,
                },
            });

        let usb = builder.build_usb();
        assert_eq!(usb[0], 0x02);
        assert_eq!(usb[1], 0xFF);
        assert_eq!(usb[2], 0x15);
        assert_eq!(usb[2] & 0x08, 0); // RELEASE_LEDS must NOT be set
        assert_eq!(usb[9], 0x01);
        assert_eq!(usb[39], 0x02);
        assert_eq!(usb[42], 0x02);
        assert_eq!(usb[43], 0x00);
        assert_eq!(usb[44], 0x0A);
        assert_eq!(usb[45], 255);
        assert_eq!(usb[46], 0);
        assert_eq!(usb[47], 0);

        assert_eq!(usb[11], 0x25); // Weapon mode is 0x25
        assert_eq!(usb[12], 2);
        assert_eq!(usb[13], 6);
        assert_eq!(usb[14], 7);

        assert_eq!(usb[22], 0x21); // Feedback mode is 0x21
        assert_eq!(usb[23], 1);
        assert_eq!(usb[24], 5);
    }

    #[test]
    fn test_bluetooth_report_structure_and_crc() {
        let builder = OutputReportBuilder::new()
            .with_lightbar(RgbColor::GREEN)
            .with_player_led(PlayerLed::Player1)
            .with_seq(3);

        let bt = builder.build_bluetooth();
        assert_eq!(bt[0], 0x31);
        assert_eq!(bt[1], 0x32);
        assert_eq!(bt[2], 0x10);
        assert_eq!(bt[4], 0x15);
        assert_eq!(bt[4] & 0x08, 0); // RELEASE_LEDS must NOT be set
        assert_eq!(bt[41], 0x02);
        assert_eq!(bt[44], 0x02);
        assert_eq!(bt[46], 0x04);
        assert_eq!(bt[48], 255);

        let crc = LittleEndian::read_u32(&bt[74..78]);
        assert_ne!(crc, 0);

        let mut hasher = Hasher::new();
        hasher.update(&[0xA2]);
        hasher.update(&bt[0..74]);
        assert_eq!(hasher.finalize(), crc);
    }

    #[test]
    fn test_parse_input_report_basic() {
        let mut raw = [0u8; 64];
        raw[0] = 0x01;
        raw[1] = 128;
        raw[2] = 128;
        raw[3] = 200;
        raw[4] = 100;
        raw[5] = 50;
        raw[6] = 250;
        raw[8] = 0x20;
        raw[9] = 0x01;

        let state = parse_input_report(&raw, ConnectionType::Usb).unwrap();
        assert_eq!(state.left_stick_x, 128);
        assert_eq!(state.right_stick_x, 200);
        assert_eq!(state.l2_trigger, 50);
        assert_eq!(state.r2_trigger, 250);
        assert!(state.cross);
        assert!(!state.circle);
        assert!(state.l1);
        assert!(!state.r1);
    }

    #[test]
    fn test_parse_input_report_bluetooth_compact() {
        // Bluetooth compact report (10 bytes)
        // [0x01, lx, ly, rx, ry, buttons1, buttons2, buttons3, l2, r2]
        let mut raw = [0u8; 10];
        raw[0] = 0x01;
        raw[1] = 64; // lx
        raw[2] = 192; // ly
        raw[3] = 220; // rx
        raw[4] = 40; // ry
        raw[5] = 0x20; // Cross (0x20) + Dpad Up (0)
        raw[6] = 0x01 | 0x10; // L1 (0x01) + Create (0x10)
        raw[7] = 0x01; // PS button (0x01)
        raw[8] = 150; // l2 analog
        raw[9] = 230; // r2 analog

        let state = parse_input_report(&raw, ConnectionType::Bluetooth).unwrap();
        assert_eq!(state.left_stick_x, 64);
        assert_eq!(state.left_stick_y, 192);
        assert_eq!(state.right_stick_x, 220);
        assert_eq!(state.right_stick_y, 40);
        assert!(state.cross);
        assert!(!state.circle);
        assert!(state.dpad_up);
        assert!(!state.dpad_down);
        assert!(state.l1);
        assert!(!state.r1);
        assert!(state.create);
        assert!(!state.options);
        assert!(state.ps_button);
        assert_eq!(state.l2_trigger, 150);
        assert_eq!(state.r2_trigger, 230);
    }

    #[test]
    fn test_parse_input_report_bluetooth_extended_0x31() {
        let mut raw = [0u8; 78];
        raw[0] = 0x31;
        raw[1] = 0x00;
        raw[2] = 128; // lx
        raw[3] = 128; // ly
        raw[4] = 210; // rx
        raw[5] = 90; // ry
        raw[6] = 80; // l2
        raw[7] = 220; // r2
        raw[8] = 5; // seq
        raw[9] = 0x40 | 0x02; // Circle (0x40) + Dpad Right (2)
        raw[10] = 0x02 | 0x20; // R1 (0x02) + Options (0x20)
        raw[11] = 0x02 | 0x04; // Touchpad click (0x02) + Mic mute (0x04)

        let state = parse_input_report(&raw, ConnectionType::Bluetooth).unwrap();
        assert_eq!(state.left_stick_x, 128);
        assert_eq!(state.right_stick_x, 210);
        assert_eq!(state.l2_trigger, 80);
        assert_eq!(state.r2_trigger, 220);
        assert!(state.circle);
        assert!(!state.cross);
        assert!(state.dpad_right);
        assert!(!state.dpad_up);
        assert!(state.r1);
        assert!(!state.l1);
        assert!(state.options);
        assert!(state.touchpad_click);
        assert!(state.mic_mute);
    }
}
