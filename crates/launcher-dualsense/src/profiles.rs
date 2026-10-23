use serde::{Deserialize, Serialize};

use crate::lightbar::RgbColor;
use crate::triggers::{DualTriggerConfig, TriggerEffect};

/// Pre-configured profile for game genres or specific titles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub triggers: DualTriggerConfig,
    pub lightbar: RgbColor,
}

impl TriggerProfile {
    pub fn racing() -> Self {
        Self {
            id: "racing_abs".to_string(),
            name: "Racing (ABS Brake & Throttle Resistance)".to_string(),
            description: "Simulates ABS anti-lock brake pulsing on L2 and progressive engine throttle resistance on R2."
                .to_string(),
            triggers: DualTriggerConfig {
                left: TriggerEffect::Vibration {
                    position: 4,
                    amplitude: 8,
                    frequency: 25,
                },
                right: TriggerEffect::Feedback {
                    start_position: 1,
                    force: 4,
                },
            },
            lightbar: RgbColor::RED,
        }
    }

    pub fn fps_tactical() -> Self {
        Self {
            id: "fps_tactical".to_string(),
            name: "FPS Tactical Shooter (Aim Wall + Trigger Snap)".to_string(),
            description:
                "Tactical aim resistance wall on L2 and a crisp mechanical weapon break on R2."
                    .to_string(),
            triggers: DualTriggerConfig {
                left: TriggerEffect::Feedback {
                    start_position: 2,
                    force: 5,
                },
                right: TriggerEffect::Weapon {
                    start_position: 2,
                    end_position: 6,
                    force: 8,
                },
            },
            lightbar: RgbColor::CYAN,
        }
    }

    pub fn flight_sim() -> Self {
        Self {
            id: "flight_sim".to_string(),
            name: "Flight Simulator (Aerodynamic Resistance)".to_string(),
            description:
                "Gradual progressive aerodynamic resistance on rudder and engine thruster controls."
                    .to_string(),
            triggers: DualTriggerConfig {
                left: TriggerEffect::Feedback {
                    start_position: 0,
                    force: 6,
                },
                right: TriggerEffect::Feedback {
                    start_position: 0,
                    force: 6,
                },
            },
            lightbar: RgbColor::SONY_BLUE,
        }
    }

    pub fn ps_classic() -> Self {
        Self {
            id: "ps_classic".to_string(),
            name: "PlayStation Classic (Linear Trigger)".to_string(),
            description:
                "Smooth, standard linear analog triggers without artificial hardware resistance."
                    .to_string(),
            triggers: DualTriggerConfig {
                left: TriggerEffect::Off,
                right: TriggerEffect::Off,
            },
            lightbar: RgbColor::SONY_BLUE,
        }
    }

    pub fn default_profiles() -> Vec<Self> {
        vec![
            Self::fps_tactical(),
            Self::racing(),
            Self::flight_sim(),
            Self::ps_classic(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profiles_integrity() {
        let profiles = TriggerProfile::default_profiles();
        assert_eq!(profiles.len(), 4);
        assert_eq!(profiles[0].id, "fps_tactical");
        assert_eq!(profiles[1].id, "racing_abs");
        assert_eq!(profiles[2].id, "flight_sim");
        assert_eq!(profiles[3].id, "ps_classic");

        for p in profiles {
            assert!(!p.name.is_empty());
            assert!(!p.description.is_empty());
        }
    }
}
