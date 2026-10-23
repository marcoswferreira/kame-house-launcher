use crate::models::{SunshineApp, SunshineAppsList};
use launcher_core::models::Game;
use std::fs;
use std::path::Path;

pub fn sync_games_to_apps_json(
    apps_json_path: &Path,
    games: &[Game],
    launcher_executable: &str,
) -> std::io::Result<SunshineAppsList> {
    let mut apps = Vec::new();

    // 1. Aplicativo Principal: Kame House Launcher (Big Picture / 10-foot UI)
    apps.push(SunshineApp {
        name: "Kame House Launcher (Console)".to_string(),
        output: None,
        cmd: Some(format!(r#""{}" --fullscreen"#, launcher_executable)),
        index: Some(-1),
        image_path: None,
        auto_detach: Some("true".to_string()),
        wait_all: Some("true".to_string()),
        exit_timeout: Some("5".to_string()),
        prep_cmd: None,
        working_dir: None,
        exclude_global_prep_cmd: None,
        elevated: None,
    });

    // 2. Jogos individuais sincronizados
    for game in games {
        if game.hidden {
            continue;
        }
        let (cmd, working_dir) = if let Some(custom) = &game.custom_exe {
            (
                Some(format!(
                    r#""{}" {}"#,
                    custom,
                    game.custom_args.as_deref().unwrap_or("")
                )),
                Path::new(custom)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string()),
            )
        } else {
            (
                Some(format!(r#""{}""#, game.path)),
                Path::new(&game.path)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string()),
            )
        };

        apps.push(SunshineApp {
            name: game.title.clone(),
            output: None,
            cmd,
            index: Some(-1),
            image_path: game.art.cover.clone().or_else(|| game.art.hero.clone()),
            auto_detach: Some("true".to_string()),
            wait_all: Some("true".to_string()),
            exit_timeout: Some("5".to_string()),
            prep_cmd: None,
            working_dir,
            exclude_global_prep_cmd: None,
            elevated: None,
        });
    }

    let list = SunshineAppsList { env: None, apps };

    if let Some(parent) = apps_json_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_vec_pretty(&list)?;
    fs::write(apps_json_path, data)?;

    Ok(list)
}
