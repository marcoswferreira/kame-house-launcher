# Steam Companion 🎮

> **Companheiro para o Ecossistema Steam & Extensão DualSense Studio de Alta Performance.**  
> Desenvolvido do zero em **100% Rust Nativo**, o Steam Companion conecta jogos fora da Steam e emuladores diretamente ao ecossistema da Valve com sincronização atômica do `shortcuts.vdf`, notificações nativas de conquistas da Steam, gerenciamento na Bandeja do Sistema (`System Tray`) e a extensão opcional **DualSense Studio**.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](LICENSE)
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
  - [2. Interoperabilidade com a Steam (`shortcuts.vdf`)](#2-interoperabilidade-com-a-steam-shortcutsvdf)
  - [3. Conquistas com Notificação Nativa na Steam](#3-conquistas-com-notificação-nativa-na-steam)
  - [4. DualSense Studio (Extensão de Hardware Opt-In)](#4-dualsense-studio-extensão-de-hardware-opt-in)
  - [5. Bandeja do Sistema (System Tray) & Terminal de Debug sob Demanda](#5-bandeja-do-sistema-system-tray--terminal-de-debug-sob-demanda)
- [Pré-requisitos e Dependências](#-pré-requisitos-e-dependências)
- [Como Compilar e Executar](#-como-compilar-e-executar)
- [Testes e Verificação](#-testes-e-verificação)
- [Decisões de Arquitetura (ADRs)](#-decisões-de-arquitetura-adrs)
- [Contribuição](#-contribuição)
- [Licença](#-licença)

---

## 🔥 Destaques

- ⚡ **100% Rust Nativo:** Consumo de memória ultrabaixo (~40MB de RAM), deixando CPU e GPU livres para seus jogos e emuladores.
- 🔗 **Ponte de Biblioteca com a Steam:** Identifica ROMs, ISOs e executáveis de PC pelo cabeçalho binário interno e sincroniza automaticamente no arquivo binário `shortcuts.vdf` da Steam com AppIDs CRC32 determinísticos e artes de Grid.
- 🏆 **Relay Nativo de Conquistas da Steam:** Dispara o aviso oficial da Steam com áudio via Chrome DevTools Protocol (CDP) quando conquistas são desbloqueadas em emuladores ou jogos avulsos.
- 🧩 **DualSense Studio (Extensão Opt-In):** Central modular de controle de hardware para controles Sony DualSense e DualSense Edge via USB e Bluetooth. Permanece **desligada (`OFF`) por padrão** para não consumir recursos até que o usuário a ative ou marque para iniciar junto com a aplicação.
- 🖥️ **Inicialização Limpa & System Tray:** Abre exclusivamente a janela do frontend sem exibir terminal em segundo plano. Ao clicar no botão fechar (`X`), permite **Minimizar para a Bandeja do Sistema (System Tray)** ou **Encerrar a Aplicação**, além de oferecer **Terminal de Debug (`Logs`)** sob demanda.

---

## 🏛 Arquitetura do Workspace

O projeto é organizado como um workspace Cargo modular, garantindo separação estrita de responsabilidades, testabilidade e reutilização:

```text
kame-house-launcher/
├── Cargo.toml                      # Manifesto do Workspace Rust
├── Cargo.lock                      # Lockfile de dependências determinísticas
├── LICENSE                         # Licença GNU General Public License v3.0
├── README.md                       # Documentação em Inglês (padrão)
├── README.pt-BR.md                 # Documentação em Português
├── CONTRIBUTING.md                 # Guia de contribuição e convenções
├── frontend/                       # Interface React 19 + TypeScript embutida no launcher-app
├── docs/                           # Documentação técnica e ADRs
│   ├── ARCHITECTURE.md             # Engenharia e diagramas de fluxo
│   └── adrs/                       # Architectural Decision Records
│       ├── 0001-rust-tauri-architecture.md
│       ├── 0003-game-scanner-and-rom-identification.md
│       └── 0004-steam-injection-and-native-notifications.md
└── crates/
    ├── launcher-core/              # Scanner multi-thread, headers de cartuchos, leitor de discos e plataformas
    ├── launcher-steam/             # VDF binário (shortcuts.vdf), artes de grid e bridge CDP WebSocket
    ├── launcher-achievements/      # Gestor unificado de conquistas e parser de logs
    ├── launcher-dualsense/         # Protocolo HID puro em Rust, 19 modos de gatilho, RGB e UDP :6969
    ├── launcher-console/           # Controle de áudio/microfone e supervisor de execução de jogos
    └── launcher-app/               # Serviço central, servidor HTTP embarcado, System Tray e CLI
```

---

## 🎮 Funcionalidades Principais

### 1. Varredura e Identificação por Conteúdo
Em vez de depender de nomes de arquivo frágeis, o `launcher-core` inspeciona o conteúdo binário:
- **Cartuchos:** Cabeçalhos de Nintendo GBA, NDS, Game Boy/Color, Nintendo 64 (com suporte a big-endian, byte-swapped e word-swapped), Sega Mega Drive/Genesis e SNES (verificação de checksum/complement LoROM e HiROM).
- **Imagens de Disco:** Extração direta de setores ISO9660 PVD, localização de `SYSTEM.CNF` de PS1/PS2 e decodificação de `PARAM.SFO` (PSP, PS3, PS4).
- **Jogos de PC:** Leitura de metadados PE (`ProductName`, `FileDescription`) e descoberta local de AppID em `steam_appid.txt`, `steam_emu.ini` e manifestos.
- **Multithreading:** Varredura paralela ultrarrápida com `rayon` e `walkdir`.

### 2. Interoperabilidade com a Steam (`shortcuts.vdf`)
- **Manipulação Binária Atômica de VDF:** O módulo `launcher-steam` codifica e decodifica o formato binário `shortcuts.vdf` da Valve sem corromper atalhos existentes.
- **Cálculo Determinístico de AppID:** Gera o hash CRC32 com o bit 31 habilitado:
  ```text
  shortcut_id = (CRC32(exe + AppName) | 0x80000000)
  ```
- **Gestão de Artes do Grid:** Posiciona capas verticais (`p.png`), banners horizontais (`_hero.png`), logos transparentes (`_logo.png`), wide (`.png`) e ícones executáveis (`_icon.ico`) na pasta `userdata/<ID>/config/grid`.

### 3. Conquistas com Notificação Nativa na Steam
- Conecta-se à porta de depuração do CEF da Steam (`127.0.0.1:8080`) via Chrome DevTools Protocol (CDP).
- Quando uma conquista é desbloqueada em emuladores ou jogos avulsos, o launcher aciona a API de notificações do contexto principal da Steam (`SharedJSContext`), exibindo o toast oficial com som e animação.

### 4. DualSense Studio (Extensão de Hardware Opt-In)
- **Desligada por Padrão (Opt-In):** A extensão não inicia junto com a aplicação por padrão, evitando qualquer consumo de recursos ou polling USB/Bluetooth em segundo plano. O usuário pode ativá-la a qualquer momento na aba **DualSense Studio** e, se desejar, marcar **"Iniciar automaticamente junto com a aplicação"**.
- **Telemetria Real de Conexão e Bateria:** Exibe o estado real do controle (`DESCONECTADO` quando nenhum controle está plugado, ou `CONECTADO` via `USB`/`BLUETOOTH` com a porcentagem exata de bateria quando conectado).
- **19 Modos de Gatilhos Adaptáveis:** Emissão direta de relatórios Sony HID (USB `0x02` e Bluetooth `0x31` com CRC32 IEEE 802.3 e desbloqueio via Feature Report `0x05`) com suporte a `Off`, `Feedback`, `Weapon`, `Bow`, `Galloping`, `MachineGun`, `Machine`, `Vibration`, `GameCube`, `Choppy`, `VerySoft`, `Soft`, `Medium`, `Hard`, `VeryHard`, `Hardest`, `Rigid`, `VibrateTriggerPulse` e curvas personalizadas de 10 zonas (`CustomZone`).
- **Barra de Luz RGB, LEDs de Jogador e Mudo do Microfone:** Efeitos em tempo real (`Static`, `Breathing`, `Rainbow`, `Battery`), LEDs indicadores de Jogador 1 a 4 e sincronização do botão físico de mudo do microfone com o áudio do sistema.
- **Testador de Controle, Emulação & Servidor UDP (:6969):** Testador visual de botões e analógicos em tempo real, ponte direta WebHID, perfis de emulação XInput/DS4, mira por giroscópio de 6 eixos e servidor UDP assíncrono na porta `6969`.

### 5. Bandeja do Sistema (System Tray) & Terminal de Debug sob Demanda
- **Apenas a Janela do Frontend:** Na inicialização, o terminal nativo permanece oculto e somente a janela da aplicação é exibida.
- **Ícone na Bandeja do Sistema (System Tray):** Permite reabrir a interface principal com duplo clique, alternar a exibição do **Terminal de Debug (`Logs`)** para diagnóstico em tempo real ou encerrar o processo.
- **Confirmação ao Fechar (`X`):** Ao clicar no botão `X`, o usuário é perguntado se deseja **Minimizar para a Bandeja (System Tray)** mantendo o serviço ativo ou **Encerrar a Aplicação** por completo.

---

## 🛠 Pré-requisitos e Dependências

- **Rust:** Versão 1.75 ou superior (`rustc` e `cargo`).
- **Node.js & npm:** Versão 20+ (necessário apenas ao recompilar o pacote `frontend/`).
- **Compilador C/C++:** `gcc`/`g++` e `pkg-config` (Linux) ou MSVC Build Tools (Windows).
- **Sistema Operacional:**
  - Windows 10/11 (64-bit).
  - Linux (Ubuntu 22.04+, Debian, Fedora, Arch Linux).

---

## 🚀 Como Compilar e Executar

### 1. Clonar o Repositório
```bash
git clone https://github.com/marcoswferreira/kame-house-launcher.git
cd kame-house-launcher
```

### 2. Compilar o Frontend (Opcional — Os ativos já estão embutidos)
```bash
npm install --prefix frontend
npm run build --prefix frontend
```

### 3. Compilar em Modo Release
```bash
cargo build --release --workspace
```
O executável otimizado estará localizado em `target/release/kame-house-launcher` (ou `target\release\kame-house-launcher.exe` no Windows).

### 4. Executar o Launcher
```bash
cargo run -p launcher-app
```

### 5. CLI e Internacionalização (i18n)
O aplicativo possui o **Inglês (`en`) como padrão** e suporte completo a **Português (`pt-BR`)**:

```bash
# Exibir status no idioma padrão (Inglês)
./target/release/kame-house-launcher status

# Salvar Português do Brasil como idioma preferencial (persistido em settings.json)
./target/release/kame-house-launcher --set-lang pt-br

# Executar temporariamente em Português
./target/release/kame-house-launcher -l pt-br status

# Exibir os comandos e ajuda da CLI
./target/release/kame-house-launcher -l pt-br --help
```

---

## 🧪 Testes e Verificação

### Executar a Suíte de Testes
```bash
cargo test --workspace
```
Todos os 53 testes unitários e de integração do workspace passam:
```text
test result: ok. 53 passed; 0 failed; 0 ignored
```

### Verificar Lints e Formatação
```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

---

## 📚 Decisões de Arquitetura (ADRs)

Para entender a motivação de cada decisão técnica, consulte a pasta [`docs/adrs/`](docs/adrs/):
- [ADR 0001: Arquitetura do Workspace em Rust e UI Embarcada](docs/adrs/0001-rust-tauri-architecture.pt-BR.md)
- [ADR 0003: Motor de Varredura de Jogos, Identificação por Conteúdo e Metadados](docs/adrs/0003-game-scanner-and-rom-identification.pt-BR.md)
- [ADR 0004: Interoperabilidade com a Steam, Injeção VDF e Notificações Nativas](docs/adrs/0004-steam-injection-and-native-notifications.pt-BR.md)

---

## 🤝 Contribuição

Contribuições são muito bem-vindas! Consulte o arquivo [`CONTRIBUTING.md`](CONTRIBUTING.md) para detalhes sobre nosso fluxo de desenvolvimento, convenções de código e o padrão de **Conventional Commits**.

---

## 📄 Licença

Este projeto é software livre licenciado sob os termos da **GNU General Public License v3.0** (GPL-3.0-or-later). Consulte o arquivo [LICENSE](LICENSE) para obter a íntegra da licença.
