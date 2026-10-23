use launcher_achievements::AchievementManager;
use launcher_console::{ConsoleSettings, LaunchSupervisor, WindowState};
use launcher_core::models::{Game, Language};
use launcher_core::scanner::scan_roots_parallel;
use launcher_steam::vdf::{encode_binary_vdf, parse_binary_vdf, upsert_shortcut, SteamShortcut};
use launcher_sunshine::{sync_games_to_apps_json, SunshineClient};
use parking_lot::RwLock;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct LauncherService {
    pub data_dir: PathBuf,
    pub games: Arc<RwLock<Vec<Game>>>,
    pub settings: Arc<RwLock<ConsoleSettings>>,
    pub achievements: Arc<RwLock<AchievementManager>>,
    pub sunshine: SunshineClient,
}

impl LauncherService {
    pub fn new(data_dir: PathBuf, sunshine_url: &str) -> Self {
        fs::create_dir_all(&data_dir).ok();
        let ach_dir = data_dir.join("achievements");
        let achievements = Arc::new(RwLock::new(AchievementManager::new(ach_dir)));

        let mut svc = Self {
            data_dir: data_dir.clone(),
            games: Arc::new(RwLock::new(Vec::new())),
            settings: Arc::new(RwLock::new(ConsoleSettings::default())),
            achievements,
            sunshine: SunshineClient::new(sunshine_url),
        };
        svc.load_state();
        svc
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

    pub fn set_language(&self, lang: Language) {
        self.settings.write().language = lang;
        let _ = self.save_state();
    }

    pub fn export_games_to_steam(&self, steam_shortcuts_vdf_path: &Path) -> std::io::Result<usize> {
        let mut root_nodes = if steam_shortcuts_vdf_path.is_file() {
            let bytes = fs::read(steam_shortcuts_vdf_path)?;
            parse_binary_vdf(&bytes)?
        } else {
            Vec::new()
        };

        let games = self.games.read().clone();
        let mut exported = 0;
        for game in &games {
            if game.hidden {
                continue;
            }
            let shortcut = SteamShortcut {
                appid: game.shortcut_id.unwrap_or(0),
                app_name: game.title.clone(),
                exe: game.custom_exe.clone().unwrap_or_else(|| game.path.clone()),
                start_dir: Path::new(&game.path)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default(),
                launch_options: game.custom_args.clone().unwrap_or_default(),
                icon: game.art.icon.clone().unwrap_or_default(),
                tags: vec![
                    "Kame House Launcher".to_string(),
                    game.platform_name.clone(),
                ],
            };
            upsert_shortcut(&mut root_nodes, shortcut);
            exported += 1;
        }

        if let Some(parent) = steam_shortcuts_vdf_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let encoded = encode_binary_vdf(&root_nodes)?;
        fs::write(steam_shortcuts_vdf_path, encoded)?;
        Ok(exported)
    }

    pub fn sync_sunshine_catalog(
        &self,
        apps_json_path: &Path,
        launcher_bin: &str,
    ) -> std::io::Result<usize> {
        let games = self.games.read().clone();
        let list = sync_games_to_apps_json(apps_json_path, &games, launcher_bin)?;
        Ok(list.apps.len())
    }
}
