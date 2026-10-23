# Kame House Launcher 🐢🎮

> **Transforme seu computador na melhor experiência de console de sala de estar.**  
> Um launcher de jogos de alta performance, completamente desenvolvido em **Rust**, com integração nativa ao **Sunshine** para streaming sem atrito e uma sessão dedicada de **Modo Console**.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
[![Coverage](https://img.shields.io/badge/coverage-82.13%25-brightgreen.svg?style=flat-square)](#testes-e-cobertura-de-código)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=flat-square)](https://conventionalcommits.org)

<p align="center">
  <a href="README.md">English</a> · <b>Português (Brasil)</b>
</p>

---

## 📋 Sumário
- [Destaques](#-destaques)
- [Arquitetura do Workspace](#-arquitetura-do-workspace)
- [Funcionalidades Principais](#-funcionalidades-principais)
  - [1. Varredura e Identificação por Conteúdo](#1-varredura-e-identificação-por-conteúdo)
  - [2. Integração Headless com Sunshine](#2-integração-headless-com-sunshine)
  - [3. Sessão Modo Console (Living Room PC)](#3-sessão-modo-console-living-room-pc)
  - [4. Interoperabilidade com Steam](#4-interoperabilidade-com-steam)
  - [5. Conquistas com Notificação Nativa](#5-conquistas-com-notificação-nativa)
- [Pré-requisitos e Dependências](#-pré-requisitos-e-dependências)
- [Como Compilar e Executar](#-como-compilar-e-executar)
- [Testes e Cobertura de Código](#-testes-e-cobertura-de-código)
- [Decisões de Arquitetura (ADRs)](#-decisões-de-arquitetura-adrs)
- [Créditos e Atribuições](#-créditos-e-atribuições)
- [Licença](#-licença)

---

## 🔥 Destaques

- ⚡ **100% Rust Nativo:** Consumo de memória ultrabaixo (~40MB de RAM contra centenas de MB de launchers tradicionais em Electron), liberando toda a CPU e GPU para os seus jogos.
- 📺 **Modo Console para Sala:** Auto-login sem senha no Windows/Linux, inicialização silenciosa em tela cheia (Kiosk), presets para TV 4K a 120Hz, ativação de HDR e roteamento automático de áudio para HDMI/Soundbar.
- 🎮 **Navegação 10-Pés & Gamepad:** Navegação completa por controle (D-pad, analógicos, gatilhos), efeitos sonoros táteis e teclado virtual em tela (OSK).
- 🔄 **Dois Modos de Lançamento:** Escolha entre **ocultar o launcher durante a gameplay e restaurar em tela cheia com foco imediato no controle** (padrão de console) ou **manter o launcher aberto em segundo plano**.
- 📡 **Sunshine Integrado (Adeus Interface Web de Navegador):** Gerenciamento headless do servidor Sunshine diretamente pelo launcher, com pop-up de pareamento por PIN de 4 dígitos para clientes Moonlight e sincronização automática de jogos para o `apps.json`.
- 🏆 **Conquistas da Steam:** Relay de notificações oficiais com áudio via Chrome DevTools Protocol (CDP), exibindo o aviso oficial da Steam para jogos de emuladores e PC.

---

## 🏛 Arquitetura do Workspace

O projeto é organizado como um workspace Cargo modular, garantindo separação estrita de responsabilidades, testabilidade e reutilização:

```text
kame-house-launcher/
├── Cargo.toml                      # Manifesto do Workspace Rust
├── Cargo.lock                      # Lockfile de dependências determinísticas
├── LICENSE                         # Licença GNU General Public License v3.0
├── README.md                       # Documentação principal
├── CONTRIBUTING.md                 # Guia de contribuição e convenções
├── Sunshine/                       # Submódulo Git oficial do Sunshine (LizardByte)
├── docs/                           # Documentação técnica e ADRs
│   ├── ARCHITECTURE.md             # Engenharia e diagramas de fluxo
│   ├── CONSOLE_MODE.md             # Guia do Modo Console
│   ├── SUNSHINE_INTEGRATION.md     # Guia da integração Sunshine
│   └── adrs/                       # Architectural Decision Records
│       ├── 0001-rust-tauri-architecture.md
│       ├── 0002-sunshine-integration-and-upstream-tracking.md
│       ├── 0003-game-scanner-and-rom-identification.md
│       ├── 0004-steam-injection-and-native-notifications.md
│       └── 0005-console-mode-transformation.md
└── crates/
    ├── launcher-core/              # Scanner, headers de cartuchos, leitor de discos e plataformas
    ├── launcher-steam/             # VDF binário (shortcuts.vdf), artes de grid e bridge CDP WebSocket
    ├── launcher-achievements/      # Gestor unificado de conquistas e parser de logs (RPCS3/RA)
    ├── launcher-sunshine/          # Cliente REST HTTPS Sunshine, pareamento PIN e sync apps.json
    ├── launcher-console/           # Display TV, áudio, auto-login, kiosk e launch supervisor
    └── launcher-app/               # Serviço central e executável kame-house-launcher
```

---

## 🎮 Funcionalidades Principais

### 1. Varredura e Identificação por Conteúdo
Em vez de depender de nomes de arquivo frágeis (como `jogo_v2_final.iso`), o `launcher-core` inspeciona o conteúdo binário:
- **Cartuchos:** Cabeçalhos de Nintendo GBA, NDS, Game Boy/Color, Nintendo 64 (com suporte a big-endian, byte-swapped e word-swapped), Sega Mega Drive/Genesis e SNES (verificação de checksum/complement LoROM e HiROM).
- **Imagens de Disco:** Extração direta de setores ISO9660 PVD, localização de `SYSTEM.CNF` de PS1/PS2 e decodificação de `PARAM.SFO` (PSP, PS3, PS4).
- **Jogos de PC e Lojas:** Leitura de metadados PE (`ProductName`, `FileDescription`) e busca de evidências de AppID em `steam_appid.txt`, `steam_emu.ini`, Goldberg e manifestos de instalação.
- **Multithreading:** Varredura paralela ultrarrápida com `rayon` e `walkdir`.

### 2. Integração Headless com Sunshine
- **Zero Apropriação:** O código de captura, codificação de vídeo (NVENC, AMF, QuickSync) e RTSP do Sunshine permanece 100% sob os cuidados do time original (LizardByte). O repositório é rastreado como submódulo Git.
- **Frontend Próprio Embutido:** O usuário não precisa abrir um navegador nem aceitar certificados SSL autoassinados. Quando o Moonlight em uma TV, Steam Deck ou celular solicita conexão, um pop-up surge na tela do launcher para inserção do PIN de 4 dígitos.
- **Sincronização de Catálogo:** Exporta os jogos adicionados e o próprio Kame House Launcher em modo Big Picture diretamente para o arquivo `apps.json` do Sunshine.

### 3. Sessão Modo Console (Living Room PC)
Projetada para eliminar as fricções de um sistema operacional de desktop tradicional:
- **Assistente de Auto-Login:** Gera as diretivas seguras de `AutoAdminLogon` no Windows ou autostart no Linux para pular a tela de senha.
- **Modo Kiosk / Shell:** Opção de iniciar diretamente na interface do launcher em tela cheia, ocultando barra de tarefas e ícones do sistema.
- **Alternador de TV & Display:** Presets de um clique para resoluções de sala: 4K 120Hz, 1440p 120Hz, 1080p 120Hz e ativação de HDR.
- **Roteamento de Áudio:** Alterna a saída de som padrão automaticamente para o HDMI da TV ou Soundbar.
- **Menu de Energia via Controle:** Ações de Suspender, Hibernar, Reiniciar ou Desligar o PC com o gamepad.
- **Dois Comportamentos de Lançamento Configuáveis:**
  - `HideAndRestoreFullscreen` (Padrão): Ao iniciar um jogo, o launcher se esconde ou minimiza para economizar recursos; ao fechar o jogo, o launcher restaura imediatamente em tela cheia com foco no controle.
  - `KeepOpen`: Mantém a interface do launcher aberta em segundo plano.

### 4. Interoperabilidade com Steam
- **Manipulação Binária Atômica de VDF:** O módulo `launcher-steam` codifica e decodifica o formato proprietário `shortcuts.vdf` da Valve sem corromper atalhos já existentes.
- **Cálculo de AppID da Steam:** Gera o hash CRC32 com o bit 31 habilitado:
  ```text
  shortcut_id = (CRC32(exe + AppName) | 0x80000000)
  ```
- **Gestão de Artes do Grid:** Posiciona capas verticais (`p.png`), banners horizontais (`_hero.png`), logos transparentes (`_logo.png`), wide (`.png`) e ícones executáveis (`_icon.ico`) na pasta `userdata/<ID>/config/grid`.

### 5. Conquistas com Notificação Nativa
- Conecta-se à porta de depuração do CEF da Steam (`127.0.0.1:8080`) via Chrome DevTools Protocol (CDP).
- Quando uma conquista é desbloqueada em emuladores ou lojas de PC, o launcher aciona a API de notificações do contexto principal da Steam (`SharedJSContext`), exibindo o toast oficial com som e animação.

---

## 🛠 Pré-requisitos e Dependências

- **Rust:** Versão 1.75 ou superior (`rustc` e `cargo`).
- **Compilador C/C++:** `gcc`/`g++` e `pkg-config` (Linux) ou MSVC Build Tools (Windows).
- **Sistema Operacional:**
  - Windows 10/11 (64-bit).
  - Linux (Ubuntu 22.04+, Debian, Fedora, Arch Linux).

---

## 🚀 Como Compilar e Executar

### 1. Clonar o Repositório com Submódulos
```bash
git clone --recurse-submodules https://github.com/marcoswferreira/kame-house-launcher.git
cd kame-house-launcher
```

Se já clonou sem os submódulos:
```bash
git submodule update --init --recursive
```

### 2. Compilar em Modo Debug
```bash
cargo build --workspace
```

### 3. Compilar em Modo Release (Otimizado para Produção)
```bash
cargo build --release --workspace
```
O executável otimizado estará localizado em `target/release/kame-house-launcher`.

### 4. Executar o Launcher
```bash
cargo run -p launcher-app
```
Ou diretamente pelo binário compilado:
```bash
./target/release/kame-house-launcher
```

### 5. CLI e Internacionalização (i18n)
O aplicativo possui o **Inglês (`en`) como padrão** e suporte completo nativo a **Português (`pt-BR`)**. Você pode alternar o idioma sob demanda por comando ou persistir sua preferência permanentemente:

```bash
# Exibir status no idioma padrão (Inglês)
./target/release/kame-house-launcher status

# Salvar Português do Brasil como idioma preferencial (persistido em settings.json)
./target/release/kame-house-launcher --set-lang pt-br

# Executar temporariamente em Português sem alterar as configurações globais
./target/release/kame-house-launcher -l pt-br status

# Retornar o idioma padrão para Inglês
./target/release/kame-house-launcher --set-lang en

# Exibir os comandos e ajuda da CLI em Português
./target/release/kame-house-launcher -l pt-br --help
```

---

## 🧪 Testes e Cobertura de Código

O projeto foi construído sob rigorosos padrões de testabilidade e possui cobertura de código superior a 80%:

### Executar a Suíte de Testes
```bash
cargo test --workspace
```
Todos os 34 testes unitários e de integração devem passar com sucesso:
```text
test result: ok. 34 passed; 0 failed; 0 ignored; finished in 0.25s
```

### Gerar Relatório de Cobertura de Código
O projeto utiliza `cargo-llvm-cov` integrado ao compilador Rust:
```bash
# Instalação da ferramenta (se necessário)
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov --locked

# Gerar resumo de cobertura do workspace
cargo llvm-cov --workspace --summary-only
```

**Resultado de Cobertura Atual:**
- **Linhas cobertas:** **82.13%**
- **Regiões cobertas:** **79.98%**
- **Funções cobertas:** **80.69%**

---

## 📚 Decisões de Arquitetura (ADRs)

Para entender a motivação de cada decisão técnica, consulte a pasta [`docs/adrs/`](docs/adrs/):
- [ADR 0001: Arquitetura do Launcher em Rust e Tauri v2](docs/adrs/0001-rust-tauri-architecture.md)
- [ADR 0002: Integração com Sunshine, Substituição do Frontend e Rastreamento Upstream](docs/adrs/0002-sunshine-integration-and-upstream-tracking.md)
- [ADR 0003: Motor de Varredura de Jogos, Identificação por Conteúdo e Metadados](docs/adrs/0003-game-scanner-and-rom-identification.md)
- [ADR 0004: Interoperabilidade com a Steam, Injeção VDF e Notificações Nativas](docs/adrs/0004-steam-injection-and-native-notifications.md)
- [ADR 0005: Sessão "Modo Console" e Transformação do PC em Console](docs/adrs/0005-console-mode-transformation.md)

---

## 🤝 Contribuição

Contribuições são muito bem-vindas! Consulte o arquivo [`CONTRIBUTING.md`](CONTRIBUTING.md) para detalhes sobre nosso fluxo de desenvolvimento, convenções de código e o padrão de **Conventional Commits**.

---

## 💖 Créditos e Atribuições

- **Sunshine:** Streaming de jogos desenvolvido e mantido pela equipe do [LizardByte & Sunshine Contributors](https://github.com/LizardByte/Sunshine) sob licença GPL-3.0. O Kame House Launcher estende a infraestrutura headless do Sunshine e concede todos os devidos créditos aos seus desenvolvedores originais.
- **Comunidade Rust & Open Source:** Agradecimentos especiais aos mantenedores das bibliotecas `tokio`, `rayon`, `reqwest`, `serde`, `byteorder`, `walkdir` e `crc32fast`.

---

## 📄 Licença

Este projeto é software livre licenciado sob os termos da **GNU General Public License v3.0** (GPL-3.0-or-later). Consulte o arquivo [LICENSE](LICENSE) para obter a íntegra da licença.

