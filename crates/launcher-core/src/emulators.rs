use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct EmulatorProfile {
    pub key: &'static str,
    pub name: &'static str,
    pub files: &'static [&'static str],
    pub default_args: &'static str,
    pub fullscreen_args: &'static str,
    pub download_url: Option<&'static str>,
}

pub static EMULATORS: &[EmulatorProfile] = &[
    EmulatorProfile {
        key: "psp",
        name: "PPSSPP",
        files: &[
            "PPSSPPWindows64.exe",
            "PPSSPPWindows.exe",
            "PPSSPPDL.exe",
            "ppsspp",
        ],
        default_args: r#""{game}""#,
        fullscreen_args: r#"--fullscreen "{game}""#,
        download_url: Some("https://www.ppsspp.org/"),
    },
    EmulatorProfile {
        key: "ps2",
        name: "PCSX2",
        files: &[
            "pcsx2-qtx64.exe",
            "pcsx2-qt.exe",
            "pcsx2x64.exe",
            "pcsx2.exe",
            "pcsx2",
        ],
        default_args: r#"-batch "{game}""#,
        fullscreen_args: r#"-batch -fullscreen "{game}""#,
        download_url: Some("https://pcsx2.net/"),
    },
    EmulatorProfile {
        key: "ps1",
        name: "DuckStation",
        files: &[
            "duckstation-qt-x64-ReleaseLTCG.exe",
            "duckstation-nogui-x64-ReleaseLTCG.exe",
            "duckstation-qt.exe",
            "duckstation",
        ],
        default_args: r#"-batch "{game}""#,
        fullscreen_args: r#"-batch -fullscreen "{game}""#,
        download_url: Some("https://www.duckstation.org/"),
    },
    EmulatorProfile {
        key: "ps3",
        name: "RPCS3",
        files: &["rpcs3.exe", "rpcs3"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"--fullscreen "{game}""#,
        download_url: Some("https://rpcs3.net/"),
    },
    EmulatorProfile {
        key: "dolphin",
        name: "Dolphin",
        files: &["Dolphin.exe", "dolphin-emu"],
        default_args: r#"-b -e "{game}""#,
        fullscreen_args: r#"-b --batch -e "{game}""#,
        download_url: Some("https://dolphin-emu.org/"),
    },
    EmulatorProfile {
        key: "cemu",
        name: "Cemu",
        files: &["Cemu.exe", "cemu"],
        default_args: r#"-g "{game}""#,
        fullscreen_args: r#"-f -g "{game}""#,
        download_url: Some("https://cemu.info/"),
    },
    EmulatorProfile {
        key: "switch",
        name: "Ryujinx / Yuzu / Citron",
        files: &[
            "Ryujinx.exe",
            "yuzu.exe",
            "citron.exe",
            "suyu.exe",
            "eden.exe",
            "sudachi.exe",
            "Ryujinx",
            "yuzu",
        ],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-f -g "{game}""#,
        download_url: None,
    },
    EmulatorProfile {
        key: "mgba",
        name: "mGBA",
        files: &["mGBA.exe", "mgba"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-f "{game}""#,
        download_url: Some("https://mgba.io/"),
    },
    EmulatorProfile {
        key: "melonds",
        name: "melonDS",
        files: &["melonDS.exe", "melonds"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-f "{game}""#,
        download_url: Some("https://melonds.kuribo64.net/"),
    },
    EmulatorProfile {
        key: "snes9x",
        name: "Snes9x",
        files: &["snes9x-x64.exe", "snes9x.exe", "snes9x"],
        default_args: r#""{game}""#,
        fullscreen_args: r#"-fullscreen "{game}""#,
        download_url: Some("https://www.snes9x.com/"),
    },
    EmulatorProfile {
        key: "retroarch",
        name: "RetroArch",
        files: &["retroarch.exe", "retroarch"],
        default_args: r#"-L {core} "{game}""#,
        fullscreen_args: r#"-f -L {core} "{game}""#,
        download_url: Some("https://www.retroarch.com/"),
    },
];

pub fn find_emulator_by_key(key: &str) -> Option<&'static EmulatorProfile> {
    EMULATORS.iter().find(|e| e.key == key)
}

pub fn guess_profile_from_exe(exe_path: &Path) -> Option<&'static EmulatorProfile> {
    let file_name = exe_path.file_name()?.to_str()?.to_ascii_lowercase();
    EMULATORS
        .iter()
        .find(|e| e.files.iter().any(|f| f.eq_ignore_ascii_case(&file_name)))
}

pub fn detected_arguments(profile_key: &str, _exe_path: &Path) -> String {
    if let Some(emu) = find_emulator_by_key(profile_key) {
        emu.fullscreen_args.to_string()
    } else {
        r#""{game}""#.to_string()
    }
}

pub fn standard_discovery_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(home) = dirs::home_dir() {
        roots.push(home.join("Emulators"));
        roots.push(home.join("Emuladores"));
        roots.push(home.join("Games"));
        roots.push(home.join("Jogos"));
    }
    #[cfg(target_os = "windows")]
    {
        roots.push(PathBuf::from(r"C:\Emulators"));
        roots.push(PathBuf::from(r"C:\Emuladores"));
        roots.push(PathBuf::from(r"C:\Jogos"));
        roots.push(PathBuf::from(r"C:\Games"));
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = dirs::home_dir() {
            roots.push(home.join(".local/share"));
            roots.push(home.join("Applications"));
        }
    }
    roots
}
