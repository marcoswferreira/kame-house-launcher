use launcher_app::cli::{execute_cli, CliArgs};
use launcher_app::LauncherService;
use std::env;
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();

    let args = match CliArgs::parse_from(env::args()) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let data_dir = env::var("KAME_HOUSE_DATA_DIR")
        .map(PathBuf::from)
        .ok()
        .or_else(|| dirs::data_dir().map(|d| d.join("kame-house-launcher")))
        .filter(|d| std::fs::create_dir_all(d).is_ok())
        .unwrap_or_else(|| {
            let fallback = PathBuf::from("./data");
            let _ = std::fs::create_dir_all(&fallback);
            fallback
        });

    let service = std::sync::Arc::new(LauncherService::new(data_dir, "https://127.0.0.1:47990"));

    execute_cli(service, args).await?;

    Ok(())
}
