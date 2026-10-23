use serde::{Deserialize, Serialize};

/// Sony DualSense Adaptive Trigger Effect Modes (Full DualSenseX 19-mode parity)
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum TriggerEffect {
    /// No adaptive resistance applied (Normal / Off)
    #[default]
    #[serde(alias = "normal")]
    Off,
    /// Continuous rigid resistance starting from specified position (Resistance / Feedback)
    #[serde(alias = "resistance")]
    Feedback {
        /// Position where resistance begins (0-9)
        start_position: u8,
        /// Resistance force (0-8)
        force: u8,
    },
    /// Weapon / Semi-Auto tactile snap (high resistance until threshold, then breaks through)
    #[serde(alias = "semi_automatic_gun")]
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
    /// Automatic Gun / Machine Gun rhythmic recoil kicks
    #[serde(alias = "automatic_gun")]
    MachineGun {
        start_position: u8,
        end_position: u8,
        strength: u8,
        frequency: u8,
    },
    /// DualSenseX Machine effect (dual-amplitude periodic oscillation)
    Machine {
        start_position: u8,
        end_position: u8,
        strength_a: u8,
        strength_b: u8,
        frequency: u8,
        period: u8,
    },
    /// Vibration / Pulse at specific position with amplitude and frequency
    #[serde(alias = "vibrate_trigger")]
    Vibration {
        position: u8,
        amplitude: u8,
        frequency: u8,
    },
    /// DualSenseX GameCube trigger (smooth analog travel ending in a hard digital click barrier)
    GameCube,
    /// DualSenseX Choppy segmented resistance steps across travel
    Choppy,
    /// DualSenseX Preset: Very Soft resistance
    VerySoft,
    /// DualSenseX Preset: Soft resistance
    Soft,
    /// DualSenseX Preset: Medium resistance
    Medium,
    /// DualSenseX Preset: Hard resistance
    Hard,
    /// DualSenseX Preset: Very Hard resistance
    VeryHard,
    /// DualSenseX Preset: Hardest resistance
    Hardest,
    /// DualSenseX Preset: Rigid full-stroke wall resistance
    Rigid,
    /// DualSenseX Preset: Vibrate Trigger Pulse
    VibrateTriggerPulse,
    /// Custom 10-zone resistance profile (CustomTriggerValue)
    #[serde(alias = "custom_trigger_value")]
    CustomZone { zones: [u8; 10] },
}

impl TriggerEffect {
    /// Returns a human-readable label for logging and diagnostics
    pub fn name(&self) -> &'static str {
        match self {
            TriggerEffect::Off => "Off/Normal",
            TriggerEffect::Feedback { .. } => "Feedback/Resistance",
            TriggerEffect::Weapon { .. } => "Weapon/SemiAutoGun",
            TriggerEffect::Bow { .. } => "Bow",
            TriggerEffect::Galloping { .. } => "Galloping",
            TriggerEffect::MachineGun { .. } => "MachineGun/AutoGun",
            TriggerEffect::Machine { .. } => "Machine",
            TriggerEffect::Vibration { .. } => "Vibration",
            TriggerEffect::GameCube => "GameCube",
            TriggerEffect::Choppy => "Choppy",
            TriggerEffect::VerySoft => "VerySoft",
            TriggerEffect::Soft => "Soft",
            TriggerEffect::Medium => "Medium",
            TriggerEffect::Hard => "Hard",
            TriggerEffect::VeryHard => "VeryHard",
            TriggerEffect::Hardest => "Hardest",
            TriggerEffect::Rigid => "Rigid",
            TriggerEffect::VibrateTriggerPulse => "VibrateTriggerPulse",
            TriggerEffect::CustomZone { .. } => "CustomTriggerValue",
        }
    }

    /// Compiles the trigger effect into the official 11-byte Sony DualSense HID payload
    pub fn compile_payload(&self) -> [u8; 11] {
        let mut bytes = [0u8; 11];
        match self {
            TriggerEffect::Off => {
                bytes[0] = 0x05;
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
                bytes[0] = 0x23;
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
            TriggerEffect::Machine {
                start_position,
                end_position,
                strength_a,
                strength_b,
                frequency,
                period,
            } => {
                bytes[0] = 0x27;
                bytes[1] = (*start_position).min(9);
                bytes[2] = (*end_position).min(9);
                bytes[3] = (*strength_a).min(7);
                bytes[4] = (*strength_b).min(7);
                bytes[5] = *frequency;
                bytes[6] = *period;
            }
            TriggerEffect::Vibration {
                position,
                amplitude,
                frequency,
            } => {
                bytes[0] = 0x06;
                bytes[1] = *frequency;
                bytes[2] = (*amplitude).min(63);
                bytes[3] = (*position).min(9);
            }
            TriggerEffect::GameCube => {
                bytes[0] = 0x02;
                bytes[1] = 0x90;
                bytes[2] = 0xA0;
                bytes[3] = 0xFF;
            }
            TriggerEffect::Choppy => {
                bytes[0] = 0x02;
                bytes[1] = 0x02;
                bytes[2] = 0x27;
                bytes[3] = 0x03;
                bytes[4] = 0x27;
                bytes[5] = 0x26;
                bytes[6] = 0x02;
            }
            TriggerEffect::VerySoft => {
                bytes[0] = 0x01;
                bytes[1] = 0x80;
                bytes[2] = 0x20;
            }
            TriggerEffect::Soft => {
                bytes[0] = 0x01;
                bytes[1] = 0x45;
                bytes[2] = 0xA0;
            }
            TriggerEffect::Medium => {
                bytes[0] = 0x02;
                bytes[1] = 0x02;
                bytes[2] = 0x01;
                bytes[3] = 0x02;
                bytes[4] = 0x02;
                bytes[5] = 0x01;
                bytes[6] = 0x22;
                bytes[7] = 0xFF;
            }
            TriggerEffect::Hard => {
                bytes[0] = 0x01;
                bytes[1] = 0x20;
                bytes[2] = 0xFF;
            }
            TriggerEffect::VeryHard => {
                bytes[0] = 0x01;
                bytes[1] = 0x10;
                bytes[2] = 0xFF;
            }
            TriggerEffect::Hardest => {
                bytes[0] = 0x01;
                bytes[1] = 0x00;
                bytes[2] = 0xFF;
                bytes[3] = 0xFF;
                bytes[4] = 0xFF;
                bytes[5] = 0xFF;
                bytes[6] = 0xFF;
            }
            TriggerEffect::Rigid => {
                bytes[0] = 0x01;
                bytes[1] = 0x00;
                bytes[2] = 0xFF;
            }
            TriggerEffect::VibrateTriggerPulse => {
                bytes[0] = 0x02;
                bytes[1] = 0x25;
                bytes[2] = 0x06;
                bytes[3] = 0x02;
                bytes[4] = 0x27;
                bytes[5] = 0x25;
                bytes[6] = 0x23;
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
        assert_eq!(payload[0], 0x05);
        assert_eq!(&payload[1..], &[0u8; 10]);
        assert_eq!(effect.name(), "Off/Normal");
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
        assert_eq!(effect.name(), "Feedback/Resistance");
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
        assert_eq!(effect.name(), "Weapon/SemiAutoGun");
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
        assert_eq!(effect.name(), "Bow");
    }

    #[test]
    fn test_trigger_galloping_and_machine() {
        let gal = TriggerEffect::Galloping {
            start_position: 1,
            end_position: 8,
            first_foot: 3,
            second_foot: 6,
            frequency: 15,
        };
        let p_gal = gal.compile_payload();
        assert_eq!(p_gal[0], 0x23);
        assert_eq!(p_gal[5], 15);
        assert_eq!(gal.name(), "Galloping");

        let mach = TriggerEffect::Machine {
            start_position: 1,
            end_position: 9,
            strength_a: 4,
            strength_b: 6,
            frequency: 20,
            period: 2,
        };
        let p_mach = mach.compile_payload();
        assert_eq!(p_mach[0], 0x27);
        assert_eq!(p_mach[5], 20);
        assert_eq!(p_mach[6], 2);
        assert_eq!(mach.name(), "Machine");
    }

    #[test]
    fn test_trigger_dualsensex_presets() {
        assert_eq!(TriggerEffect::GameCube.compile_payload()[0], 0x02);
        assert_eq!(TriggerEffect::GameCube.compile_payload()[1], 0x90);
        assert_eq!(TriggerEffect::Choppy.compile_payload()[0], 0x02);
        assert_eq!(TriggerEffect::VerySoft.compile_payload()[0], 0x01);
        assert_eq!(TriggerEffect::Soft.compile_payload()[0], 0x01);
        assert_eq!(TriggerEffect::Medium.compile_payload()[0], 0x02);
        assert_eq!(TriggerEffect::Hard.compile_payload()[0], 0x01);
        assert_eq!(TriggerEffect::VeryHard.compile_payload()[0], 0x01);
        assert_eq!(TriggerEffect::Hardest.compile_payload()[3], 0xFF);
        assert_eq!(TriggerEffect::Rigid.compile_payload()[2], 0xFF);
        assert_eq!(
            TriggerEffect::VibrateTriggerPulse.compile_payload()[0],
            0x02
        );
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
