use serde::{Deserialize, Serialize};

/// Sony DualSense Adaptive Trigger Effect Modes
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum TriggerEffect {
    /// No adaptive resistance applied
    #[default]
    Off,
    /// Continuous rigid resistance starting from specified position
    Feedback {
        /// Position where resistance begins (0-9)
        start_position: u8,
        /// Resistance force (0-8)
        force: u8,
    },
    /// Weapon / Semi-Auto tactile snap (high resistance until threshold, then breaks through)
    Weapon {
        /// Position where trigger resistance begins (0-9)
        start_position: u8,
        /// Position where trigger breaks through / fires (0-9)
        end_position: u8,
        /// Resistance force before break (0-8)
        force: u8,
    },
    /// Bow tension simulation (progressive force until release)
    Bow {
        /// Start of draw (0-9)
        start_position: u8,
        /// Fully drawn position (0-9)
        end_position: u8,
        /// Initial pull force (0-8)
        force: u8,
        /// Force at full draw (0-8)
        snap_force: u8,
    },
    /// Galloping effect with alternating resistance pulses
    Galloping {
        start_position: u8,
        end_position: u8,
        first_foot: u8,
        second_foot: u8,
        frequency: u8,
    },
    /// Machine Gun rhythmic recoil kicks
    MachineGun {
        start_position: u8,
        end_position: u8,
        strength: u8,
        frequency: u8,
    },
    /// Vibration / Pulse at specific position with amplitude and frequency
    Vibration {
        position: u8,
        amplitude: u8,
        frequency: u8,
    },
    /// Custom 10-zone resistance profile (zones 0 to 9, 0..255 force each)
    CustomZone { zones: [u8; 10] },
}

impl TriggerEffect {
    /// Compiles the trigger effect into the official 11-byte Sony DualSense HID payload
    pub fn compile_payload(&self) -> [u8; 11] {
        let mut bytes = [0u8; 11];
        match self {
            TriggerEffect::Off => {
                bytes[0] = 0x00;
            }
            TriggerEffect::Feedback {
                start_position,
                force,
            } => {
                bytes[0] = 0x21;
                bytes[1] = (*start_position).min(9);
                bytes[2] = (*force).min(8);
            }
            TriggerEffect::Weapon {
                start_position,
                end_position,
                force,
            } => {
                bytes[0] = 0x25;
                bytes[1] = (*start_position).min(9);
                bytes[2] = (*end_position).min(9);
                bytes[3] = (*force).min(8);
            }
            TriggerEffect::Bow {
                start_position,
                end_position,
                force,
                snap_force,
            } => {
                bytes[0] = 0x22;
                bytes[1] = (*start_position).min(9);
                bytes[2] = (*end_position).min(9);
                bytes[3] = (*force).min(8);
                bytes[4] = (*snap_force).min(8);
            }
            TriggerEffect::Galloping {
                start_position,
                end_position,
                first_foot,
                second_foot,
                frequency,
            } => {
                bytes[0] = 0x25;
                bytes[1] = (*start_position).min(9);
                bytes[2] = (*end_position).min(9);
                bytes[3] = (*first_foot).min(9);
                bytes[4] = (*second_foot).min(9);
                bytes[5] = *frequency;
            }
            TriggerEffect::MachineGun {
                start_position,
                end_position,
                strength,
                frequency,
            } => {
                bytes[0] = 0x26;
                bytes[1] = (*start_position).min(9);
                bytes[2] = (*end_position).min(9);
                bytes[3] = (*strength).min(8);
                bytes[4] = *frequency;
            }
            TriggerEffect::Vibration {
                position,
                amplitude,
                frequency,
            } => {
                bytes[0] = 0x21;
                bytes[1] = (*position).min(9);
                bytes[2] = (*amplitude).min(8);
                bytes[3] = *frequency;
            }
            TriggerEffect::CustomZone { zones } => {
                bytes[0] = 0x27;
                bytes[1..11].copy_from_slice(zones);
            }
        }
        bytes
    }
}

/// Dual trigger configuration for Left (L2) and Right (R2) triggers
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DualTriggerConfig {
    pub left: TriggerEffect,
    pub right: TriggerEffect,
}

impl DualTriggerConfig {
    pub fn new(left: TriggerEffect, right: TriggerEffect) -> Self {
        Self { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_off() {
        let effect = TriggerEffect::Off;
        let payload = effect.compile_payload();
        assert_eq!(payload, [0u8; 11]);
    }

    #[test]
    fn test_trigger_feedback() {
        let effect = TriggerEffect::Feedback {
            start_position: 2,
            force: 7,
        };
        let payload = effect.compile_payload();
        assert_eq!(payload[0], 0x21);
        assert_eq!(payload[1], 2);
        assert_eq!(payload[2], 7);
        assert_eq!(&payload[3..], &[0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_trigger_weapon() {
        let effect = TriggerEffect::Weapon {
            start_position: 1,
            end_position: 6,
            force: 8,
        };
        let payload = effect.compile_payload();
        assert_eq!(payload[0], 0x25);
        assert_eq!(payload[1], 1);
        assert_eq!(payload[2], 6);
        assert_eq!(payload[3], 8);
    }

    #[test]
    fn test_trigger_bow() {
        let effect = TriggerEffect::Bow {
            start_position: 0,
            end_position: 8,
            force: 4,
            snap_force: 7,
        };
        let payload = effect.compile_payload();
        assert_eq!(payload[0], 0x22);
        assert_eq!(payload[1], 0);
        assert_eq!(payload[2], 8);
        assert_eq!(payload[3], 4);
        assert_eq!(payload[4], 7);
    }

    #[test]
    fn test_trigger_custom_zone() {
        let zones = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        let effect = TriggerEffect::CustomZone { zones };
        let payload = effect.compile_payload();
        assert_eq!(payload[0], 0x27);
        assert_eq!(&payload[1..11], &zones);
    }

    #[test]
    fn test_trigger_serialization() {
        let config = DualTriggerConfig {
            left: TriggerEffect::Feedback {
                start_position: 3,
                force: 5,
            },
            right: TriggerEffect::Weapon {
                start_position: 2,
                end_position: 7,
                force: 6,
            },
        };
        let json = serde_json::to_string(&config).unwrap();
        let decoded: DualTriggerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, decoded);
    }
}
