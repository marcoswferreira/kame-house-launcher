use crate::i18n::I18n;
use crate::LauncherService;
use launcher_core::models::{Language, LaunchBehavior};
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CliCommand {
    RunDaemon { keep_alive: bool, open_ui: bool },
    Status,
    Scan { paths: Vec<PathBuf> },
    ExportSteam { path: Option<PathBuf> },
    SetLanguage { lang: Language },
    SetLaunchMode { behavior: LaunchBehavior },
    PrintHelp,
    PrintVersion,
}

#[derive(Debug)]
pub struct CliArgs {
    pub language_override: Option<Language>,
    pub command: CliCommand,
}

impl CliArgs {
    pub fn parse_from<I, T>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let items: Vec<String> = args.into_iter().map(Into::into).collect();
        let mut lang_override = None;
        let mut i = 1; // skip binary name

        // First, check env var KAME_HOUSE_LANG
        if let Ok(env_lang) = env::var("KAME_HOUSE_LANG") {
            if let Some(parsed) = Language::parse_flexible(&env_lang) {
                lang_override = Some(parsed);
            }
        }

        let mut command = CliCommand::RunDaemon {
            keep_alive: true,
            open_ui: true,
        };

        while i < items.len() {
            let arg = &items[i];
            match arg.as_str() {
                "--once" => {
                    command = CliCommand::RunDaemon {
                        keep_alive: false,
                        open_ui: false,
                    };
                }
                "--headless" => {
                    command = CliCommand::RunDaemon {
                        keep_alive: true,
                        open_ui: false,
                    };
                }
                "run" => {
                    command = CliCommand::RunDaemon {
                        keep_alive: true,
                        open_ui: true,
                    };
                }
                "-h" | "--help" => {
                    return Ok(Self {
                        language_override: lang_override,
                        command: CliCommand::PrintHelp,
                    });
                }
                "-v" | "--version" => {
                    return Ok(Self {
                        language_override: lang_override,
                        command: CliCommand::PrintVersion,
                    });
                }
                "-l" | "--lang" => {
                    i += 1;
                    if i >= items.len() {
                        return Err("Missing argument for --lang".to_string());
                    }
                    let val = &items[i];
                    lang_override = Some(Language::parse_flexible(val).ok_or_else(|| {
                        format!("Unknown language: '{}'. Expected 'en' or 'pt-BR'", val)
                    })?);
                }
                "--set-lang" => {
                    i += 1;
                    if i >= items.len() {
                        return Err("Missing argument for --set-lang".to_string());
                    }
                    let val = &items[i];
                    let parsed = Language::parse_flexible(val).ok_or_else(|| {
                        format!("Unknown language: '{}'. Expected 'en' or 'pt-BR'", val)
                    })?;
                    command = CliCommand::SetLanguage { lang: parsed };
                }
                "status" => {
                    command = CliCommand::Status;
                }
                "scan" => {
                    let mut paths = Vec::new();
                    i += 1;
                    while i < items.len() && !items[i].starts_with('-') {
                        paths.push(PathBuf::from(&items[i]));
                        i += 1;
                    }
                    command = CliCommand::Scan { paths };
                    continue;
                }
                "export-steam" => {
                    i += 1;
                    let path = if i < items.len() && !items[i].starts_with('-') {
                        Some(PathBuf::from(&items[i]))
                    } else {
                        None
                    };
                    command = CliCommand::ExportSteam { path };
                }
                "set-mode" => {
                    i += 1;
                    if i >= items.len() {
                        return Err(
                            "Missing argument for set-mode. Expected 'hide' or 'keep-open'"
                                .to_string(),
                        );
                    }
                    let mode_str = items[i].to_lowercase();
                    let behavior = match mode_str.as_str() {
                        "hide" | "hide-and-restore" | "fullscreen" => {
                            LaunchBehavior::HideAndRestoreFullscreen
                        }
                        "keep-open" | "keep" | "background" => LaunchBehavior::KeepOpen,
                        _ => {
                            return Err(format!(
                                "Unknown mode '{}'. Expected 'hide' or 'keep-open'",
                                items[i]
                            ))
                        }
                    };
                    command = CliCommand::SetLaunchMode { behavior };
                }
                unknown if !unknown.starts_with('-') => {
                    return Err(format!(
                        "Unknown command: '{}'. Run with --help for usage.",
                        unknown
                    ));
                }
                unknown => {
                    return Err(format!(
                        "Unknown option: '{}'. Run with --help for usage.",
                        unknown
                    ));
                }
            }
            i += 1;
        }

        Ok(Self {
            language_override: lang_override,
            command,
        })
    }
}

pub async fn execute_cli(service: Arc<LauncherService>, args: CliArgs) -> anyhow::Result<()> {
    let effective_lang = args.language_override.unwrap_or_else(|| service.language());
    let i18n = I18n::new(effective_lang);

    match args.command {
        CliCommand::PrintHelp => {
            println!("{}", i18n.cli_help());
        }
        CliCommand::PrintVersion => {
            println!("kame-house-launcher {}", env!("CARGO_PKG_VERSION"));
        }
        CliCommand::Status => {
            let games = service.list_games();
            let settings = service.settings.read().clone();
            println!("{}", i18n.status_title());
            println!(
                "{}: {} [{}]",
                i18n.status_language_label(),
                settings.language.display_name(),
                settings.language.code()
            );
            println!("{}: {}", i18n.status_games_label(), games.len());
            println!(
                "{}: {}",
                i18n.status_launch_behavior_label(),
                i18n.launch_behavior_desc(settings.launch_behavior)
            );
            println!("{}: UDP Link :6969 [Active]", i18n.status_dualsense_label());
            println!("Big Picture UI: http://127.0.0.1:48080");
        }
        CliCommand::Scan { paths } => {
            let roots = if paths.is_empty() {
                vec![dirs::home_dir()
                    .map(|h| h.join("Games"))
                    .unwrap_or_else(|| PathBuf::from("./games"))]
            } else {
                paths
            };
            let count = service.scan_and_add(&roots, 4);
            println!("{}", i18n.scan_completed(count));
        }
        CliCommand::ExportSteam { path } => {
            let target = path.unwrap_or_else(|| {
                dirs::data_dir()
                    .map(|d| d.join("Steam/userdata/0/config/shortcuts.vdf"))
                    .unwrap_or_else(|| PathBuf::from("./shortcuts.vdf"))
            });
            let count = service.export_games_to_steam(&target)?;
            println!(
                "{}",
                i18n.export_steam_completed(count, &target.display().to_string())
            );
        }
        CliCommand::SetLanguage { lang } => {
            service.set_language(lang);
            let updated_i18n = I18n::new(lang);
            println!("{}", updated_i18n.language_updated(lang));
        }
        CliCommand::SetLaunchMode { behavior } => {
            let mut settings = service.settings.read().clone();
            settings.launch_behavior = behavior;
            service.update_console_settings(settings);
            println!("{}", i18n.launch_mode_updated(behavior));
        }
        CliCommand::RunDaemon {
            keep_alive,
            open_ui,
        } => {
            if keep_alive && open_ui {
                crate::tray::hide_console_on_startup_if_daemon(true);
            }

            // Log with effective i18n
            tracing::info!("{}", i18n.starting_banner(env!("CARGO_PKG_VERSION")));
            let games = service.list_games();
            tracing::info!("{}", i18n.games_loaded(games.len()));
            tracing::info!("{}", i18n.dualsense_and_console_ready());
            tracing::info!("{}", i18n.active_language_msg());
            let settings = service.settings.read().clone();
            tracing::info!(
                "{}",
                i18n.launch_behavior_configured(settings.launch_behavior)
            );

            // Start DualSense UDP Link Server only if the DualSense extension is enabled on startup
            if settings.dualsense_extension_enabled && settings.dualsense_udp_enabled {
                let ds = Arc::clone(&service.dualsense);
                tokio::spawn(async move {
                    if let Err(e) = ds.start_udp().await {
                        tracing::warn!("Could not start DualSense UDP server: {}", e);
                    }
                });
            }

            // Bind HTTP Server BEFORE launching UI (guarantees socket is listening & falls back gracefully)
            let server_svc = Arc::clone(&service);
            let preferred_port = env::var("KAME_HOUSE_PORT")
                .ok()
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or(48080);

            let (server, listener, bound_port) =
                match crate::server::HttpServer::bind_with_fallback(server_svc, preferred_port)
                    .await
                {
                    Ok(res) => res,
                    Err(err) => {
                        tracing::error!(
                            "FATAL: Failed to bind Kame House Big Picture Server: {}",
                            err
                        );
                        eprintln!("Error starting HTTP server: {}", err);
                        return Err(err.into());
                    }
                };

            // Spawn the accept loop
            tokio::spawn(async move {
                if let Err(e) = server.serve(listener).await {
                    tracing::error!("Kame House HTTP server error: {}", e);
                }
            });

            let ui_url = format!("http://127.0.0.1:{}", bound_port);
            service.lifecycle.set_ui_url(ui_url.clone());
            tracing::info!("Kame House Big Picture UI: {}", ui_url);
            println!("\n{}", i18n.daemon_running_prompt());

            if keep_alive {
                crate::tray::start_system_tray(
                    Arc::clone(&service),
                    Arc::clone(&service.lifecycle),
                );
            }

            if open_ui {
                crate::server::open_fullscreen_ui(&ui_url);
            }

            if keep_alive {
                if tokio::signal::ctrl_c().await.is_err() {
                    std::future::pending::<()>().await;
                }
                crate::tray::cleanup_system_tray();
                println!("\n{}", i18n.daemon_shutting_down());
            }
        }
    }

    Ok(())
}
