use std::process::Command;

/// Returns the platform command to toggle system microphone mute state
pub fn get_toggle_mic_command() -> (&'static str, Vec<String>) {
    if cfg!(target_os = "windows") {
        (
            "powershell",
            vec![
                "-NoProfile".to_string(),
                "-NonInteractive".to_string(),
                "-Command".to_string(),
                // Toggles recording mute via Windows CoreAudio if audio module is present,
                // or falls back to volume mute keystroke emulation
                "$dev = Get-Command Set-AudioDevice -ErrorAction SilentlyContinue; if ($dev) { (Get-AudioDevice -Recording).Mute | Out-Null } else { (New-Object -ComObject WScript.Shell).SendKeys([char]173) }".to_string(),
            ],
        )
    } else {
        (
            "pactl",
            vec![
                "set-source-mute".to_string(),
                "@DEFAULT_SOURCE@".to_string(),
                "toggle".to_string(),
            ],
        )
    }
}

/// Returns the platform command to set system microphone mute state explicitly
pub fn get_set_mic_mute_command(mute: bool) -> (&'static str, Vec<String>) {
    let mute_str = if mute { "1" } else { "0" };
    if cfg!(target_os = "windows") {
        let ps_cmd = if mute {
            "$dev = Get-Command Set-AudioDevice -ErrorAction SilentlyContinue; if ($dev) { Set-AudioDevice -RecordingMute 1 } else { (New-Object -ComObject WScript.Shell).SendKeys([char]173) }"
        } else {
            "$dev = Get-Command Set-AudioDevice -ErrorAction SilentlyContinue; if ($dev) { Set-AudioDevice -RecordingMute 0 } else { (New-Object -ComObject WScript.Shell).SendKeys([char]173) }"
        };
        (
            "powershell",
            vec![
                "-NoProfile".to_string(),
                "-NonInteractive".to_string(),
                "-Command".to_string(),
                ps_cmd.to_string(),
            ],
        )
    } else {
        (
            "pactl",
            vec![
                "set-source-mute".to_string(),
                "@DEFAULT_SOURCE@".to_string(),
                mute_str.to_string(),
            ],
        )
    }
}

/// Returns the command to route default audio playback to a target device (e.g. HDMI TV or Controller)
pub fn get_switch_playback_device_command(device_name: &str) -> (&'static str, Vec<String>) {
    if cfg!(target_os = "windows") {
        (
            "powershell",
            vec![
                "-NoProfile".to_string(),
                "-NonInteractive".to_string(),
                "-Command".to_string(),
                format!(
                    "$dev = Get-Command Set-AudioDevice -ErrorAction SilentlyContinue; if ($dev) {{ Set-AudioDevice -Index (Get-AudioDevice -List | Where-Object {{ $_.Name -like '*{}*' }}).Index }}",
                    device_name
                ),
            ],
        )
    } else {
        (
            "pactl",
            vec!["set-default-sink".to_string(), device_name.to_string()],
        )
    }
}

/// Executes system microphone toggle (non-blocking safe)
pub fn execute_toggle_microphone() -> Result<(), std::io::Error> {
    let (prog, args) = get_toggle_mic_command();
    Command::new(prog).args(args).spawn().map(|_| ())
}

/// Executes system microphone explicit mute/unmute
pub fn execute_set_microphone_mute(mute: bool) -> Result<(), std::io::Error> {
    let (prog, args) = get_set_mic_mute_command(mute);
    Command::new(prog).args(args).spawn().map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_commands_generation() {
        let (prog, args) = get_toggle_mic_command();
        assert!(!prog.is_empty());
        assert!(!args.is_empty());

        let (m_prog, m_args) = get_set_mic_mute_command(true);
        assert!(!m_prog.is_empty());
        assert!(!m_args.is_empty());

        let (unm_prog, unm_args) = get_set_mic_mute_command(false);
        assert!(!unm_prog.is_empty());
        assert!(!unm_args.is_empty());

        let (dev_prog, dev_args) = get_switch_playback_device_command("Wireless Controller");
        assert!(!dev_prog.is_empty());
        assert!(!dev_args.is_empty());
        assert!(dev_args.iter().any(|a| a.contains("Wireless Controller")));
    }
}
