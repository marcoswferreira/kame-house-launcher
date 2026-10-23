pub mod emulation;
pub mod lightbar;
pub mod manager;
pub mod profiles;
pub mod protocol;
pub mod triggers;
pub mod udp_server;

pub use emulation::{EmulationConfig, EmulationMode, GyroSettings};
pub use lightbar::{LightbarMode, LightingConfig, MicLed, PlayerLed, RgbColor};
pub use manager::{DualSenseManager, DualSenseStatus};
pub use profiles::TriggerProfile;
pub use protocol::{
    BatteryState, ConnectionType, DualSenseInputState, MotionData, OutputReportBuilder, TouchPoint,
};
pub use triggers::{DualTriggerConfig, TriggerEffect};
pub use udp_server::{DualSenseUdpServer, UdpServerStats, DEFAULT_DUALSENSEX_UDP_PORT};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_full_crate_exports() {
        let manager = DualSenseManager::new();
        let status = manager.get_status().await;
        assert!(status.connected);
        assert_eq!(status.connection_type, ConnectionType::Usb);
    }
}
