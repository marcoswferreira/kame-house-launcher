use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Locates Steam's `userdata` directory across Windows, Linux, and macOS.
pub fn find_steam_userdata_dirs() -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    // 1. Check explicit environment overrides
    if let Ok(custom_path) = std::env::var("STEAM_PATH") {
        let p = PathBuf::from(custom_path).join("userdata");
        candidates.push(p);
    }
    if let Ok(custom_dir) = std::env::var("STEAM_DIR") {
        let p = PathBuf::from(custom_dir).join("userdata");
        candidates.push(p);
    }

    // 2. Windows registry query and known default paths
    #[cfg(target_os = "windows")]
    {
        // Query HKCU\Software\Valve\Steam via reg command
        if let Ok(output) = std::process::Command::new("reg")
            .args(["query", r"HKCU\Software\Valve\Steam", "/v", "SteamPath"])
            .output()
        {
            if output.status.success() {
                let out_str = String::from_utf8_lossy(&output.stdout);
                for line in out_str.lines() {
                    if line.contains("SteamPath") && line.contains("REG_SZ") {
                        if let Some(val) = line.split("REG_SZ").nth(1) {
                            let clean = val.trim();
                            if !clean.is_empty() {
                                candidates.push(PathBuf::from(clean).join("userdata"));
                            }
                        }
                    }
                }
            }
        }

        // Standard Windows installation directories
        candidates.push(PathBuf::from(r"C:\Program Files (x86)\Steam\userdata"));
        candidates.push(PathBuf::from(r"C:\Program Files\Steam\userdata"));
        candidates.push(PathBuf::from(r"C:\Steam\userdata"));
        candidates.push(PathBuf::from(r"D:\Steam\userdata"));
        candidates.push(PathBuf::from(r"E:\Steam\userdata"));
        candidates.push(PathBuf::from(r"D:\Games\Steam\userdata"));
        candidates.push(PathBuf::from(r"E:\Games\Steam\userdata"));
    }

    // 3. Linux / SteamOS / Steam Deck paths
    if let Some(data_dir) = dirs::data_dir() {
        candidates.push(data_dir.join("Steam/userdata"));
    }
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".local/share/Steam/userdata"));
        candidates.push(home.join(".steam/steam/userdata"));
        candidates.push(home.join(".steam/root/userdata"));
        // Flatpak Steam
        candidates.push(home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam/userdata"));
        candidates.push(home.join(".var/app/com.valvesoftware.Steam/data/Steam/userdata"));
        // Snap Steam
        candidates.push(home.join("snap/steam/common/.local/share/Steam/userdata"));
    }

    // Deduplicate while preserving order
    let mut deduped = Vec::new();
    for p in candidates {
        if !deduped.contains(&p) {
            deduped.push(p);
        }
    }

    deduped
}

/// Locates the active user's `shortcuts.vdf` file.
/// Returns `(path, is_real_steam)` where `is_real_steam` indicates whether
/// an actual Steam userdata directory was found.
pub fn find_steam_shortcuts_vdf(fallback_dir: &Path) -> (PathBuf, bool) {
    // 1. Explicit override via environment
    if let Ok(env_path) = std::env::var("STEAM_SHORTCUTS_PATH") {
        let p = PathBuf::from(env_path);
        if !p.as_os_str().is_empty() {
            info!(
                "Using custom STEAM_SHORTCUTS_PATH override: {}",
                p.display()
            );
            return (p, true);
        }
    }

    let userdata_dirs = find_steam_userdata_dirs();

    let mut best_candidate: Option<(PathBuf, std::time::SystemTime)> = None;
    let mut fallback_candidate: Option<PathBuf> = None;

    for u_dir in userdata_dirs {
        if !u_dir.is_dir() {
            continue;
        }

        let entries = match fs::read_dir(&u_dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let user_folder = entry.path();
            if !user_folder.is_dir() {
                continue;
            }

            let folder_name = user_folder
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();

            // Prefer real account IDs (numeric, non-zero)
            let is_anonymous = folder_name == "0";
            let vdf = user_folder.join("config").join("shortcuts.vdf");

            if vdf.is_file() {
                let mtime = fs::metadata(&vdf)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);

                if !is_anonymous {
                    if let Some((_, best_time)) = &best_candidate {
                        if mtime > *best_time {
                            best_candidate = Some((vdf, mtime));
                        }
                    } else {
                        best_candidate = Some((vdf, mtime));
                    }
                } else if fallback_candidate.is_none() {
                    fallback_candidate = Some(vdf);
                }
            } else if user_folder.join("config").is_dir()
                && !is_anonymous
                && fallback_candidate.is_none()
            {
                fallback_candidate = Some(vdf);
            }
        }
    }

    if let Some((best, _)) = best_candidate {
        info!(
            "Discovered primary active Steam shortcuts.vdf: {}",
            best.display()
        );
        return (best, true);
    }

    if let Some(fallback) = fallback_candidate {
        info!(
            "Discovered Steam shortcuts.vdf candidate: {}",
            fallback.display()
        );
        return (fallback, true);
    }

    // Safe fallback to application data directory (never throws OS error 5)
    let app_fallback = fallback_dir.join("shortcuts.vdf");
    debug!(
        "No active Steam installation found. Using application data directory fallback: {}",
        app_fallback.display()
    );
    (app_fallback, false)
}

/// Checks if the Steam client process is actively running in the OS
pub fn is_steam_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("tasklist")
            .args(["/FI", "IMAGENAME eq steam.exe", "/NH"])
            .output()
            .map(|out| {
                let s = String::from_utf8_lossy(&out.stdout);
                s.to_lowercase().contains("steam.exe")
            })
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("pgrep")
            .args(["-x", "steam"])
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }
}

/// Safely writes `shortcuts.vdf` content:
/// - Strips Windows Read-Only file flags if set.
/// - Creates a `.bak` backup copy of existing shortcuts.
/// - Performs atomic write via temporary file with graceful replacement.
/// - Provides a helpful, human-readable error if Steam client process lock prevents write.
pub fn safe_write_shortcuts_vdf(target_path: &Path, content: &[u8]) -> Result<()> {
    #[cfg(not(test))]
    if is_steam_running() {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "O cliente da Steam está aberto no momento. Feche a Steam completamente (inclusive na bandeja do sistema) antes de sincronizar para evitar que a Steam bloqueie ou sobrescreva os atalhos.",
        ));
    }

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 1. Remove Read-Only flag if present
    if target_path.is_file() {
        if let Ok(metadata) = fs::metadata(target_path) {
            let mut perms = metadata.permissions();
            if perms.readonly() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    perms.set_mode(0o644);
                }
                #[cfg(not(unix))]
                #[allow(clippy::permissions_set_readonly_false)]
                {
                    perms.set_readonly(false);
                }
                let _ = fs::set_permissions(target_path, perms);
                info!(
                    "Removed Read-Only attribute from: {}",
                    target_path.display()
                );
            }
        }

        // 2. Create safety backup
        let backup_path = target_path.with_extension("vdf.bak");
        if let Err(e) = fs::copy(target_path, &backup_path) {
            debug!(
                "Could not create backup shortcuts.vdf.bak (non-fatal): {}",
                e
            );
        }
    }

    // 3. Write via temporary file with robust replacement fallback
    let tmp_path = target_path.with_extension("vdf.tmp");

    match fs::write(&tmp_path, content) {
        Ok(_) => {
            // Attempt rename/replace
            if let Err(_e) = fs::rename(&tmp_path, target_path) {
                // On Windows, rename fails if destination exists and cannot be replaced directly.
                let _ = fs::remove_file(target_path);
                if let Err(_second_rename) = fs::rename(&tmp_path, target_path) {
                    // Try fallback write directly to target
                    if let Err(write_err) = fs::write(target_path, content) {
                        let _ = fs::remove_file(&tmp_path);
                        return map_access_denied_error(write_err, target_path);
                    }
                }
                let _ = fs::remove_file(&tmp_path);
            }
            Ok(())
        }
        Err(_e) => {
            let _ = fs::remove_file(&tmp_path);
            // Fallback direct write
            match fs::write(target_path, content) {
                Ok(_) => Ok(()),
                Err(direct_err) => map_access_denied_error(direct_err, target_path),
            }
        }
    }
}

fn map_access_denied_error(err: Error, path: &Path) -> Result<()> {
    if err.kind() == ErrorKind::PermissionDenied || err.raw_os_error() == Some(5) {
        warn!(
            "Access denied writing to {}. Steam client might be running with exclusive locks.",
            path.display()
        );
        Err(Error::new(
            ErrorKind::PermissionDenied,
            format!(
                "Acesso negado ao gravar '{}'. Por favor, feche o cliente da Steam completamente antes de sincronizar os atalhos (Access is denied / Steam client file lock).",
                path.display()
            ),
        ))
    } else {
        Err(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_find_steam_userdata_dirs_returns_list() {
        let dirs = find_steam_userdata_dirs();
        assert!(!dirs.is_empty(), "Should return standard candidates list");
    }

    #[test]
    fn test_find_steam_shortcuts_fallback_dir() {
        let temp = tempdir().unwrap();
        let (path, is_real) = find_steam_shortcuts_vdf(temp.path());
        assert!(!path.as_os_str().is_empty());
        if !is_real {
            assert_eq!(path, temp.path().join("shortcuts.vdf"));
        }
    }

    #[test]
    fn test_safe_write_shortcuts_vdf_roundtrip_and_backup() {
        let temp = tempdir().unwrap();
        let target = temp.path().join("config").join("shortcuts.vdf");

        let initial_data = b"Initial VDF Data";
        safe_write_shortcuts_vdf(&target, initial_data).unwrap();
        assert_eq!(fs::read(&target).unwrap(), initial_data);

        // Update data - should create .bak
        let updated_data = b"Updated VDF Data";
        safe_write_shortcuts_vdf(&target, updated_data).unwrap();
        assert_eq!(fs::read(&target).unwrap(), updated_data);

        let backup = target.with_extension("vdf.bak");
        assert!(backup.is_file());
        assert_eq!(fs::read(&backup).unwrap(), initial_data);
    }

    #[test]
    fn test_safe_write_removes_readonly_attribute() {
        let temp = tempdir().unwrap();
        let target = temp.path().join("shortcuts.vdf");

        safe_write_shortcuts_vdf(&target, b"Read only test").unwrap();
        assert!(target.is_file());

        // Set read-only
        let mut perms = fs::metadata(&target).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&target, perms).unwrap();

        // Safe write should strip read-only and succeed
        let new_data = b"Overwritten successfully";
        safe_write_shortcuts_vdf(&target, new_data).unwrap();
        assert_eq!(fs::read(&target).unwrap(), new_data);
    }
}
