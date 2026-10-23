use serde::{Deserialize, Serialize};

/// RGB Color representation for DualSense Lightbar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self {
        r: 0,
        g: 120,
        b: 255,
    };
    pub const AMBER: Self = Self {
        r: 255,
        g: 140,
        b: 0,
    };
    pub const PURPLE: Self = Self {
        r: 160,
        g: 32,
        b: 240,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const SONY_BLUE: Self = Self {
        r: 0,
        g: 68,
        b: 204,
    };

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }
}

impl Default for RgbColor {
    fn default() -> Self {
        Self::SONY_BLUE
    }
}

/// Player LED Indicator beneath the touchpad (Player 1 to 4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerLed {
    Off,
    #[default]
    Player1, // Center dot
    Player2, // 2 dots
    Player3, // 3 dots
    Player4, // 4 dots
    All,     // 5 dots
}

impl PlayerLed {
    /// Sony bitmask for player LEDs
    pub fn to_bitmask(&self) -> u8 {
        match self {
            PlayerLed::Off => 0x00,
            PlayerLed::Player1 => 0x04, // Center LED (0000 0100)
            PlayerLed::Player2 => 0x0A, // 2 LEDs (0000 1010)
            PlayerLed::Player3 => 0x15, // 3 LEDs (0001 0101)
            PlayerLed::Player4 => 0x1B, // 4 LEDs (0001 1011)
            PlayerLed::All => 0x1F,     // 5 LEDs (0001 1111)
        }
    }
}

/// Microphone Mute LED
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MicLed {
    #[default]
    Off,
    Solid,
    Pulse,
}

impl MicLed {
    pub fn to_byte(&self) -> u8 {
        match self {
            MicLed::Off => 0x00,
            MicLed::Solid => 0x01,
            MicLed::Pulse => 0x02,
        }
    }
}

/// Lightbar lighting mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum LightbarMode {
    Off,
    Static { color: RgbColor },
    Breathing { color: RgbColor, speed: f32 },
    Rainbow { speed: f32 },
    BatteryLevel,
}

impl Default for LightbarMode {
    fn default() -> Self {
        LightbarMode::Static {
            color: RgbColor::SONY_BLUE,
        }
    }
}

impl LightbarMode {
    /// Computes active RGB color for this moment in time (seconds) and battery status
    pub fn compute_color(&self, elapsed_secs: f32, battery_pct: u8, is_charging: bool) -> RgbColor {
        match self {
            LightbarMode::Off => RgbColor::BLACK,
            LightbarMode::Static { color } => *color,
            LightbarMode::Breathing { color, speed } => {
                let factor = (elapsed_secs * speed * std::f32::consts::PI).sin().abs();
                RgbColor {
                    r: ((color.r as f32) * factor).round() as u8,
                    g: ((color.g as f32) * factor).round() as u8,
                    b: ((color.b as f32) * factor).round() as u8,
                }
            }
            LightbarMode::Rainbow { speed } => {
                let hue = (elapsed_secs * speed * 60.0).rem_euclid(360.0);
                hsv_to_rgb(hue, 1.0, 1.0)
            }
            LightbarMode::BatteryLevel => {
                if is_charging {
                    // Pulsing amber when charging
                    let pulse = (elapsed_secs * 3.0).sin().abs();
                    RgbColor {
                        r: 255,
                        g: (140.0 * pulse).round() as u8,
                        b: 0,
                    }
                } else if battery_pct >= 60 {
                    RgbColor::GREEN
                } else if battery_pct >= 20 {
                    RgbColor::AMBER
                } else {
                    RgbColor::RED
                }
            }
        }
    }
}

/// Full lighting configuration
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LightingConfig {
    pub lightbar: LightbarMode,
    pub player_led: PlayerLed,
    pub mic_led: MicLed,
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> RgbColor {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    RgbColor {
        r: ((r_prime + m) * 255.0).round() as u8,
        g: ((g_prime + m) * 255.0).round() as u8,
        b: ((b_prime + m) * 255.0).round() as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_hex_conversion() {
        let color = RgbColor::new(255, 128, 0);
        assert_eq!(color.to_hex(), "#ff8000");

        let parsed = RgbColor::from_hex("#ff8000").unwrap();
        assert_eq!(parsed, color);

        let parsed_no_hash = RgbColor::from_hex("00ff00").unwrap();
        assert_eq!(parsed_no_hash, RgbColor::GREEN);

        assert!(RgbColor::from_hex("invalid").is_none());
    }

    #[test]
    fn test_player_led_bitmasks() {
        assert_eq!(PlayerLed::Off.to_bitmask(), 0x00);
        assert_eq!(PlayerLed::Player1.to_bitmask(), 0x04);
        assert_eq!(PlayerLed::Player2.to_bitmask(), 0x0A);
        assert_eq!(PlayerLed::Player3.to_bitmask(), 0x15);
        assert_eq!(PlayerLed::Player4.to_bitmask(), 0x1B);
        assert_eq!(PlayerLed::All.to_bitmask(), 0x1F);
    }

    #[test]
    fn test_lightbar_battery_colors() {
        let mode = LightbarMode::BatteryLevel;
        assert_eq!(mode.compute_color(0.0, 90, false), RgbColor::GREEN);
        assert_eq!(mode.compute_color(0.0, 45, false), RgbColor::AMBER);
        assert_eq!(mode.compute_color(0.0, 10, false), RgbColor::RED);
    }

    #[test]
    fn test_lightbar_breathing_and_rainbow() {
        let breathing = LightbarMode::Breathing {
            color: RgbColor::WHITE,
            speed: 1.0,
        };
        let c1 = breathing.compute_color(0.5, 100, false);
        assert!(c1.r > 200);

        let rainbow = LightbarMode::Rainbow { speed: 1.0 };
        let r1 = rainbow.compute_color(0.0, 100, false);
        assert_eq!(r1, RgbColor::RED);
    }
}
