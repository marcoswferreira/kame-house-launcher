use launcher_console::TvResolution;
use launcher_core::models::{Language, LaunchBehavior};

/// Central Internationalization engine for Kame House Launcher.
/// Default: English (en) with full Portuguese (pt-BR) support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I18n {
    pub language: Language,
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            language: Language::En,
        }
    }
}

impl I18n {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn starting_banner(&self, version: &str) -> String {
        match self.language {
            Language::En => format!("Starting Kame House Launcher v{}...", version),
            Language::PtBr => format!("Iniciando Kame House Launcher v{}...", version),
        }
    }

    pub fn games_loaded(&self, count: usize) -> String {
        match self.language {
            Language::En => format!(
                "Kame House Launcher loaded with {} games in catalog.",
                count
            ),
            Language::PtBr => format!(
                "Kame House Launcher carregado com {} jogos no catálogo.",
                count
            ),
        }
    }

    pub fn sunshine_and_console_ready(&self) -> &'static str {
        match self.language {
            Language::En => "Sunshine control plane and Console Mode Session ready.",
            Language::PtBr => "Plano de controle Sunshine e Sessão Modo Console prontos.",
        }
    }

    pub fn active_language_msg(&self) -> String {
        match self.language {
            Language::En => format!(
                "Active language: {} [{}]",
                self.language.display_name(),
                self.language.code()
            ),
            Language::PtBr => format!(
                "Idioma ativo: {} [{}]",
                self.language.display_name(),
                self.language.code()
            ),
        }
    }

    pub fn launch_behavior_desc(&self, behavior: LaunchBehavior) -> &'static str {
        match self.language {
            Language::En => match behavior {
                LaunchBehavior::HideAndRestoreFullscreen => {
                    "Hide on launch and restore in fullscreen with focus on exit (Console Default)"
                }
                LaunchBehavior::KeepOpen => "Keep launcher open in background",
            },
            Language::PtBr => match behavior {
                LaunchBehavior::HideAndRestoreFullscreen => {
                    "Ocultar ao abrir e restaurar em tela cheia com foco no fechamento (Padrão Console)"
                }
                LaunchBehavior::KeepOpen => "Manter o launcher aberto em segundo plano",
            },
        }
    }

    pub fn launch_behavior_configured(&self, behavior: LaunchBehavior) -> String {
        let desc = self.launch_behavior_desc(behavior);
        match self.language {
            Language::En => format!("Configured launch behavior: {}", desc),
            Language::PtBr => format!("Comportamento de inicialização configurado: {}", desc),
        }
    }

    pub fn power_action_name(&self, action: &str) -> &'static str {
        match self.language {
            Language::En => match action {
                "sleep" => "Sleep",
                "hibernate" => "Hibernate",
                "restart" => "Restart",
                "shutdown" => "Shutdown",
                _ => "Unknown",
            },
            Language::PtBr => match action {
                "sleep" => "Suspender",
                "hibernate" => "Hibernar",
                "restart" => "Reiniciar",
                "shutdown" => "Desligar",
                _ => "Desconhecido",
            },
        }
    }

    pub fn tv_resolution_desc(&self, res: TvResolution) -> String {
        let (w, h, hz) = res.dimensions();
        match self.language {
            Language::En => format!("{}x{} @ {}Hz (TV Preset)", w, h, hz),
            Language::PtBr => format!("{}x{} a {}Hz (Perfil de TV)", w, h, hz),
        }
    }

    pub fn cli_help(&self) -> String {
        match self.language {
            Language::En => [
                "Kame House Launcher - Living Room Console Game Launcher",
                "",
                "USAGE:",
                "    kame-house-launcher [OPTIONS] [COMMAND]",
                "",
                "OPTIONS:",
                "    -l, --lang <en|pt-br>        Override language for this run (default: en)",
                "        --set-lang <en|pt-br>    Set and persist preferred language",
                "    -h, --help                   Print help information",
                "    -v, --version                Print version information",
                "",
                "COMMANDS:",
                "    status                       Show launcher status, catalog, and configuration",
                "    scan [PATH...]               Scan directories for games and update catalog",
                "    export-steam [PATH]          Export shortcuts into Valve binary shortcuts.vdf",
                "    sync-sunshine [PATH]         Synchronize game catalog into Sunshine apps.json",
                "    set-mode <hide|keep-open>    Set game launch behavior (hide & restore vs keep open)",
                "",
                "ENVIRONMENT:",
                "    KAME_HOUSE_LANG              Override default language via environment (e.g., 'en' or 'pt-BR')",
            ]
            .join("\n"),
            Language::PtBr => [
                "Kame House Launcher - Launcher Gamer de Sala de Estar em Modo Console",
                "",
                "USO:",
                "    kame-house-launcher [OPÇÕES] [COMANDO]",
                "",
                "OPÇÕES:",
                "    -l, --lang <en|pt-br>        Define o idioma para esta execução (padrão: en)",
                "        --set-lang <en|pt-br>    Define e salva o idioma preferencial permanentemente",
                "    -h, --help                   Exibe informações de ajuda",
                "    -v, --version                Exibe a versão do aplicativo",
                "",
                "COMANDOS:",
                "    status                       Exibe status do launcher, catálogo e configurações",
                "    scan [DIRETÓRIO...]          Varre diretórios procurando jogos e atualiza catálogo",
                "    export-steam [CAMINHO]       Exporta atalhos para o shortcuts.vdf binário da Steam",
                "    sync-sunshine [CAMINHO]      Sincroniza o catálogo de jogos no apps.json do Sunshine",
                "    set-mode <hide|keep-open>    Define o comportamento de lançamento (ocultar vs manter aberto)",
                "",
                "VARIÁVEIS DE AMBIENTE:",
                "    KAME_HOUSE_LANG              Sobrescreve o idioma padrão via ambiente (ex: 'en' ou 'pt-BR')",
            ]
            .join("\n"),
        }
    }

    pub fn scan_completed(&self, count: usize) -> String {
        match self.language {
            Language::En => format!("Scan completed: found and registered {} game(s).", count),
            Language::PtBr => format!(
                "Varredura concluída: {} jogo(s) encontrado(s) e registrado(s).",
                count
            ),
        }
    }

    pub fn export_steam_completed(&self, count: usize, path: &str) -> String {
        match self.language {
            Language::En => format!(
                "Successfully exported {} shortcut(s) to Steam VDF: {}",
                count, path
            ),
            Language::PtBr => format!(
                "{} atalho(s) exportado(s) com sucesso para o VDF da Steam: {}",
                count, path
            ),
        }
    }

    pub fn sync_sunshine_completed(&self, count: usize, path: &str) -> String {
        match self.language {
            Language::En => format!(
                "Successfully synchronized {} app(s) to Sunshine apps.json: {}",
                count, path
            ),
            Language::PtBr => format!(
                "{} aplicativo(s) sincronizado(s) com sucesso no apps.json do Sunshine: {}",
                count, path
            ),
        }
    }

    pub fn language_updated(&self, lang: Language) -> String {
        match self.language {
            Language::En => format!(
                "Preferred language successfully updated to: {} [{}]",
                lang.display_name(),
                lang.code()
            ),
            Language::PtBr => format!(
                "Idioma de preferência atualizado com sucesso para: {} [{}]",
                lang.display_name(),
                lang.code()
            ),
        }
    }

    pub fn launch_mode_updated(&self, behavior: LaunchBehavior) -> String {
        let desc = self.launch_behavior_desc(behavior);
        match self.language {
            Language::En => format!("Launch mode successfully updated to: {}", desc),
            Language::PtBr => format!(
                "Modo de inicialização atualizado com sucesso para: {}",
                desc
            ),
        }
    }

    pub fn status_title(&self) -> &'static str {
        match self.language {
            Language::En => "=== Kame House Launcher Status ===",
            Language::PtBr => "=== Status do Kame House Launcher ===",
        }
    }

    pub fn status_language_label(&self) -> &'static str {
        match self.language {
            Language::En => "Active Language",
            Language::PtBr => "Idioma Ativo",
        }
    }

    pub fn status_games_label(&self) -> &'static str {
        match self.language {
            Language::En => "Catalog Games",
            Language::PtBr => "Jogos no Catálogo",
        }
    }

    pub fn status_launch_behavior_label(&self) -> &'static str {
        match self.language {
            Language::En => "Launch Behavior",
            Language::PtBr => "Comportamento de Execução",
        }
    }

    pub fn status_sunshine_label(&self) -> &'static str {
        match self.language {
            Language::En => "Sunshine Endpoint",
            Language::PtBr => "Ponto de Acesso Sunshine",
        }
    }
}
