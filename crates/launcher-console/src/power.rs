use crate::models::PowerAction;

pub fn get_power_command(action: PowerAction) -> (&'static str, &'static [&'static str]) {
    #[cfg(target_os = "windows")]
    {
        match action {
            PowerAction::Sleep => ("rundll32.exe", &["powrprof.dll,SetSuspendState", "0,1,0"]),
            PowerAction::Hibernate => ("shutdown", &["/h"]),
            PowerAction::Restart => ("shutdown", &["/r", "/t", "0"]),
            PowerAction::Shutdown => ("shutdown", &["/s", "/t", "0"]),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        match action {
            PowerAction::Sleep => ("systemctl", &["suspend"]),
            PowerAction::Hibernate => ("systemctl", &["hibernate"]),
            PowerAction::Restart => ("systemctl", &["reboot"]),
            PowerAction::Shutdown => ("systemctl", &["poweroff"]),
        }
    }
}
