use launcher_achievements::AchievementManager;
use launcher_console::{ConsoleSettings, LaunchSupervisor, WindowState};
use launcher_core::models::{Game, Language};
use launcher_core::scanner::scan_roots_parallel;
use launcher_dualsense::DualSenseManager;
use launcher_steam::discovery::{find_steam_shortcuts_vdf, safe_write_shortcuts_vdf};
use launcher_steam::vdf::{encode_binary_vdf, parse_binary_vdf, upsert_shortcut, SteamShortcut};
use parking_lot::RwLock;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct LauncherService {
    pub data_dir: PathBuf,
    pub games: Arc<RwLock<Vec<Game>>>,
    pub settings: Arc<RwLock<ConsoleSettings>>,
    pub achievements: Arc<RwLock<AchievementManager>>,
    pub dualsense: Arc<DualSenseManager>,
    pub lifecycle: Arc<crate::tray::WindowLifecycleState>,
    pub dualsense_worker: RwLock<Option<launcher_dualsense::HidDeviceWorker>>,
}

impl LauncherService {
    pub fn new(data_dir: PathBuf) -> Self {
        fs::create_dir_all(&data_dir).ok();
        let ach_dir = data_dir.join("achievements");
        let achievements = Arc::new(RwLock::new(AchievementManager::new(ach_dir)));

        let dualsense = Arc::new(DualSenseManager::new());
        dualsense.set_mic_toggle_handler(Arc::new(|muted| {
            let _ = launcher_console::audio::execute_set_microphone_mute(muted);
        }));

        let lifecycle = crate::tray::WindowLifecycleState::default();
        lifecycle.set_log_file_path(data_dir.join("kame-house-debug.log"));

        let mut svc = Self {
            data_dir: data_dir.clone(),
            games: Arc::new(RwLock::new(Vec::new())),
            settings: Arc::new(RwLock::new(ConsoleSettings::default())),
            achievements,
            dualsense,
            lifecycle: Arc::new(lifecycle),
            dualsense_worker: RwLock::new(None),
        };
        svc.load_state();

        let autostart_ds = svc.settings.read().dualsense_extension_enabled;
        svc.dualsense.set_autostart_enabled(autostart_ds);
        if autostart_ds {
            svc.set_dualsense_extension_active(true);
        }

        svc
    }

    pub fn set_dualsense_extension_active(&self, active: bool) {
        self.dualsense.set_extension_enabled(active);
        let mut worker_slot = self.dualsense_worker.write();
        if active {
            if worker_slot.is_none() {
                let worker = launcher_dualsense::HidDeviceWorker::start(self.dualsense.clone());
                *worker_slot = Some(worker);
            }
        } else if let Some(worker) = worker_slot.take() {
            worker.stop();
        }
    }

    pub fn set_dualsense_autostart(&self, autostart: bool) {
        self.settings.write().dualsense_extension_enabled = autostart;
        self.dualsense.set_autostart_enabled(autostart);
        let _ = self.save_state();
    }

    fn games_file(&self) -> PathBuf {
        self.data_dir.join("games.json")
    }

    fn settings_file(&self) -> PathBuf {
        self.data_dir.join("settings.json")
    }

    pub fn load_state(&mut self) {
        let gfile = self.games_file();
        if gfile.is_file() {
            if let Ok(bytes) = fs::read(&gfile) {
                if let Ok(list) = serde_json::from_slice::<Vec<Game>>(&bytes) {
                    *self.games.write() = list;
                }
            }
        }

        let sfile = self.settings_file();
        if sfile.is_file() {
            if let Ok(bytes) = fs::read(&sfile) {
                if let Ok(cfg) = serde_json::from_slice::<ConsoleSettings>(&bytes) {
                    *self.settings.write() = cfg;
                }
            }
        }
    }

    pub fn save_state(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.data_dir)?;
        let gdata = serde_json::to_vec_pretty(&*self.games.read())?;
        fs::write(self.games_file(), gdata)?;

        let sdata = serde_json::to_vec_pretty(&*self.settings.read())?;
        fs::write(self.settings_file(), sdata)?;
        Ok(())
    }

    pub fn list_games(&self) -> Vec<Game> {
        self.games.read().clone()
    }

    pub fn add_game(&self, game: Game) {
        let mut list = self.games.write();
        if let Some(pos) = list.iter().position(|g| g.path == game.path) {
            list[pos] = game;
        } else {
            list.push(game);
        }
        drop(list);
        let _ = self.save_state();
    }

    pub fn scan_and_add(&self, roots: &[PathBuf], max_depth: usize) -> usize {
        let found = scan_roots_parallel(roots, max_depth);
        let count = found.len();
        let mut list = self.games.write();
        for g in found {
            if !list.iter().any(|existing| existing.path == g.path) {
                list.push(g);
            }
        }
        drop(list);
        let _ = self.save_state();
        count
    }

    pub fn prepare_launch(&self, game_id: &str) -> Option<(Game, WindowState)> {
        let games = self.games.read();
        let game = games
            .iter()
            .find(|g| g.id == game_id || g.uuid.to_string() == game_id)?
            .clone();
        drop(games);

        let mut supervisor = LaunchSupervisor::new(self.settings.read().clone());
        let window_state = supervisor.on_game_launch_prepare();
        Some((game, window_state))
    }

    pub fn finish_launch(&self, game_id: &str, played_seconds: u64) -> WindowState {
        let mut games = self.games.write();
        if let Some(g) = games
            .iter_mut()
            .find(|g| g.id == game_id || g.uuid.to_string() == game_id)
        {
            g.playtime_seconds += played_seconds;
            g.last_played_at = Some(chrono::Utc::now().to_rfc3339());
        }
        drop(games);
        let _ = self.save_state();

        let mut supervisor = LaunchSupervisor::new(self.settings.read().clone());
        supervisor.on_game_exit()
    }

    pub fn update_console_settings(&self, new_settings: ConsoleSettings) {
        *self.settings.write() = new_settings;
        let _ = self.save_state();
    }

    pub fn language(&self) -> Language {
        self.settings.read().language
    }

    pub fn remove_game(&self, game_id: &str) -> bool {
        let mut list = self.games.write();
        let initial_len = list.len();
        list.retain(|g| g.id != game_id && g.uuid.to_string() != game_id);
        let removed = list.len() < initial_len;
        drop(list);
        if removed {
            let _ = self.save_state();
        }
        removed
    }

    pub fn set_language(&self, lang: Language) {
        self.settings.write().language = lang;
        let _ = self.save_state();
    }

    pub fn get_steam_shortcuts_path(&self) -> PathBuf {
        self.get_steam_shortcuts_info().0
    }

    pub fn get_steam_shortcuts_info(&self) -> (PathBuf, bool, bool) {
        // Returns (resolved_path, is_real_steam, is_custom_user_path)
        if let Some(custom) = &self.settings.read().steam_shortcuts_path {
            if let Some(resolved) = launcher_steam::resolve_steam_shortcuts_path(custom) {
                return (resolved, true, true);
            }
        }
        let (path, is_real) = find_steam_shortcuts_vdf(&self.data_dir);
        (path, is_real, false)
    }

    pub fn set_steam_shortcuts_path(&self, custom: Option<String>) {
        let mut settings = self.settings.read().clone();
        settings.steam_shortcuts_path = custom;
        self.update_console_settings(settings);
    }

    pub fn update_game(&self, updated: Game) -> bool {
        let mut list = self.games.write();
        if let Some(pos) = list
            .iter()
            .position(|g| g.id == updated.id || g.uuid == updated.uuid)
        {
            list[pos] = updated;
            drop(list);
            let _ = self.save_state();
            true
        } else {
            false
        }
    }

    pub fn get_configured_emulators(&self) -> std::collections::HashMap<String, String> {
        self.settings.read().emulator_paths.clone()
    }

    pub fn set_configured_emulator(&self, profile_key: String, path: String) {
        let mut settings = self.settings.read().clone();
        if path.trim().is_empty() {
            settings.emulator_paths.remove(&profile_key);
        } else {
            settings.emulator_paths.insert(profile_key, path);
        }
        self.update_console_settings(settings);
    }

    pub fn export_games_to_steam(&self, steam_shortcuts_vdf_path: &Path) -> std::io::Result<usize> {
        let mut root_nodes = if steam_shortcuts_vdf_path.is_file() {
            // Strip Read-Only flag if set before reading
            if let Ok(metadata) = fs::metadata(steam_shortcuts_vdf_path) {
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
                    let _ = fs::set_permissions(steam_shortcuts_vdf_path, perms);
                }
            }
            let bytes = fs::read(steam_shortcuts_vdf_path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::PermissionDenied || e.raw_os_error() == Some(5) {
                    std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        format!(
                            "Acesso negado ao ler '{}'. Por favor feche a Steam antes de sincronizar.",
                            steam_shortcuts_vdf_path.display()
                        ),
                    )
                } else {
                    e
                }
            })?;
            parse_binary_vdf(&bytes)?
        } else {
            Vec::new()
        };

        let grid_dir = steam_shortcuts_vdf_path
            .parent()
            .map(|p| p.join("grid"))
            .unwrap_or_else(|| steam_shortcuts_vdf_path.with_file_name("grid"));
        let grid_mgr = launcher_steam::SteamGridManager::new(grid_dir);

        let games = self.games.read().clone();
        let emulator_map = self.settings.read().emulator_paths.clone();
        let mut exported = 0;
        for game in &games {
            if game.hidden {
                continue;
            }

            let (exe, launch_options, start_dir) =
                launcher_core::emulators::resolve_game_launch_command(game, &emulator_map);

            let mut icon_path = game.art.icon.clone().unwrap_or_default();
            if icon_path.is_empty() && exe.to_ascii_lowercase().ends_with(".exe") {
                icon_path = exe.clone();
            }

            let shortcut = SteamShortcut {
                appid: game.shortcut_id.unwrap_or(0),
                app_name: game.title.clone(),
                exe,
                start_dir,
                launch_options,
                icon: icon_path,
                tags: vec!["Steam Companion".to_string(), game.platform_name.clone()],
            };
            let shortcut_id = upsert_shortcut(&mut root_nodes, shortcut);

            // Export artwork to steam grid
            if let Some(cover) = &game.art.cover {
                let p = Path::new(cover);
                if p.is_file() {
                    let _ = grid_mgr.copy_art_file(shortcut_id, "cover", p);
                }
            }
            if let Some(hero) = &game.art.hero {
                let p = Path::new(hero);
                if p.is_file() {
                    let _ = grid_mgr.copy_art_file(shortcut_id, "hero", p);
                }
            }
            if let Some(logo) = &game.art.logo {
                let p = Path::new(logo);
                if p.is_file() {
                    let _ = grid_mgr.copy_art_file(shortcut_id, "logo", p);
                }
            }
            if let Some(wide) = &game.art.wide {
                let p = Path::new(wide);
                if p.is_file() {
                    let _ = grid_mgr.copy_art_file(shortcut_id, "wide", p);
                }
            }
            if let Some(icon) = &game.art.icon {
                let p = Path::new(icon);
                if p.is_file() && !icon.to_ascii_lowercase().ends_with(".exe") {
                    let _ = grid_mgr.copy_art_file(shortcut_id, "icon", p);
                }
            }

            exported += 1;
        }

        let encoded = encode_binary_vdf(&root_nodes)?;
        safe_write_shortcuts_vdf(steam_shortcuts_vdf_path, &encoded)?;
        Ok(exported)
    }
}
