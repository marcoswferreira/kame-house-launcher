use launcher_app::LauncherService;
use launcher_core::models::LaunchBehavior;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();

    info!("Iniciando Kame House Launcher v0.1.0...");

    let data_dir = dirs::data_dir()
        .map(|d| d.join("kame-house-launcher"))
        .unwrap_or_else(|| PathBuf::from("./data"));

    let service = LauncherService::new(data_dir, "https://127.0.0.1:47990");

    let games = service.list_games();
    info!("Kame House Launcher carregado com {} jogos no catálogo.", games.len());
    info!("Plano de controle Sunshine e Sessão Modo Console prontos.");

    let settings = service.settings.read().clone();
    let mode_desc = match settings.launch_behavior {
        LaunchBehavior::HideAndRestoreFullscreen => "Ocultar ao abrir e restaurar em tela cheia com foco no fechamento (Padrão Console)",
        LaunchBehavior::KeepOpen => "Manter o launcher aberto em segundo plano",
    };
    info!("Comportamento de inicialização configurado: {}", mode_desc);

    Ok(())
}

