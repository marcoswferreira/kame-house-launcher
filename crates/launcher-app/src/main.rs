#![cfg_attr(not(test), windows_subsystem = "windows")]

use launcher_app::cli::{execute_cli, CliArgs, CliCommand};
use launcher_app::LauncherService;
use parking_lot::Mutex;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
struct TeeLogMakeWriter {
    file: Option<Arc<Mutex<File>>>,
}

struct TeeLogWriter {
    file: Option<Arc<Mutex<File>>>,
}

impl Write for TeeLogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = io::stdout().write_all(buf);
        if let Some(ref f) = self.file {
            let mut guard = f.lock();
            let _ = guard.write_all(buf);
            let _ = guard.flush();
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let _ = io::stdout().flush();
        if let Some(ref f) = self.file {
            let _ = f.lock().flush();
        }
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for TeeLogMakeWriter {
    type Writer = TeeLogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        TeeLogWriter {
            file: self.file.clone(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let args = match CliArgs::parse_from(env::args()) {
        Ok(a) => a,
        Err(err) => {
            launcher_app::tray::attach_parent_console_for_cli();
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    if matches!(
        args.command,
        CliCommand::RunDaemon {
            keep_alive: true,
            open_ui: true
        }
    ) {
        launcher_app::tray::hide_console_on_startup_if_daemon(true);
    } else {
        launcher_app::tray::attach_parent_console_for_cli();
    }

    let log_file_path = data_dir.join("kame-house-debug.log");
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_file_path)
        .ok()
        .map(|f| Arc::new(Mutex::new(f)));

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(cfg!(not(target_os = "windows")))
        .with_writer(TeeLogMakeWriter { file: log_file })
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();

    let service = Arc::new(LauncherService::new(data_dir));

    execute_cli(service, args).await?;

    Ok(())
}
