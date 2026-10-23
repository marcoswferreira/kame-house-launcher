use std::fs;
use std::path::{Path, PathBuf};

pub fn generate_linux_autostart_desktop_file(exec_path: &str) -> String {
    format!(
        "[Desktop Entry]\n\
        Type=Application\n\
        Version=1.0\n\
        Name=Kame House Launcher\n\
        Comment=Console Living Room Game Launcher\n\
        Exec={} --fullscreen\n\
        StartupNotify=false\n\
        Terminal=false\n\
        Categories=Game;\n",
        exec_path
    )
}

pub fn save_linux_autostart(autostart_dir: &Path, exec_path: &str) -> std::io::Result<PathBuf> {
    fs::create_dir_all(autostart_dir)?;
    let file_path = autostart_dir.join("kame-house-launcher.desktop");
    let content = generate_linux_autostart_desktop_file(exec_path);
    fs::write(&file_path, content)?;
    Ok(file_path)
}

pub fn generate_windows_autologin_reg(username: &str, domain: Option<&str>) -> String {
    format!(
        "Windows Registry Editor Version 5.00\n\n\
        [HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Winlogon]\n\
        \"AutoAdminLogon\"=\"1\"\n\
        \"DefaultUserName\"=\"{}\"\n\
        \"DefaultDomainName\"=\"{}\"\n",
        username,
        domain.unwrap_or(".")
    )
}
