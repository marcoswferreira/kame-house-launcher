use serde::{Deserialize, Serialize};

/// Controller Emulation Target Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmulationMode {
    /// Pure native DualSense HID (Steam Input / PS5 native games)
    #[default]
    Native,
    /// Emulate Xbox 360 controller via XInput (ViGEmBus / uinput)
    Xbox360,
    /// Emulate DualShock 4 controller
    DualShock4,
}

/// Gyro Aiming and Motion Mapping Settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GyroSettings {
    pub enabled: bool,
    /// Gyro sensitivity multiplier (0.1 to 5.0, default 1.0)
    pub sensitivity: f32,
    /// Deadzone threshold for noise elimination
    pub deadzone: i16,
    pub invert_x: bool,
    pub invert_y: bool,
}

impl Default for GyroSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            sensitivity: 1.0,
            deadzone: 50,
            invert_x: false,
            invert_y: false,
        }
    }
}

impl GyroSettings {
    /// Maps raw 16-bit gyro yaw/pitch into stick deflection delta (-32768..32767)
    pub fn map_gyro_to_stick_delta(&self, raw_yaw: i16, raw_pitch: i16) -> (i16, i16) {
        if !self.enabled {
            return (0, 0);
        }

        let filter = |val: i16, invert: bool| -> i16 {
            if val.abs() < self.deadzone {
                return 0;
            }
            let adjusted = ((val as f32) * self.sensitivity).clamp(-32768.0, 32767.0) as i16;
            if invert {
                -adjusted
            } else {
                adjusted
            }
        };

        (
            filter(raw_yaw, self.invert_x),
            filter(raw_pitch, self.invert_y),
        )
    }
}

/// Full controller emulation and mapping configuration
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EmulationConfig {
    pub mode: EmulationMode,
    pub gyro: GyroSettings,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emulation_mode_defaults() {
        let config = EmulationConfig::default();
        assert_eq!(config.mode, EmulationMode::Native);
        assert!(!config.gyro.enabled);
    }

    #[test]
    fn test_gyro_mapping() {
        let mut gyro = GyroSettings::default();
        // Disabled -> 0
        assert_eq!(gyro.map_gyro_to_stick_delta(500, -300), (0, 0));

        gyro.enabled = true;
        // Below deadzone
        assert_eq!(gyro.map_gyro_to_stick_delta(30, -20), (0, 0));

        // Active
        let (dx, dy) = gyro.map_gyro_to_stick_delta(200, -400);
        assert_eq!(dx, 200);
        assert_eq!(dy, -400);

        // Inverted
        gyro.invert_x = true;
        gyro.invert_y = true;
        let (idx, idy) = gyro.map_gyro_to_stick_delta(200, -400);
        assert_eq!(idx, -200);
        assert_eq!(idy, 400);
    }
}
