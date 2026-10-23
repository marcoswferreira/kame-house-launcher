# Arquitetura do Sistema: Kame House Launcher 🐢

Este documento detalha o design de software, a divisão de responsabilidades das crates, os fluxos de dados e as integrações de sistema do **Kame House Launcher**.

---

## 1. Visão Geral da Arquitetura

O Kame House Launcher adota uma arquitetura em camadas orientada a eventos e crates modulares em Rust:

```mermaid
flowchart TD
    subgraph UI["Camada de Interface (React 19 Steam Deck Theme)"]
        Grid["Biblioteca de Jogos (Grid & Detalhes)"]
        DualSenseTab["DualSense Studio (Gatilhos, RGB, Link UDP)"]
        ConsoleTab["Sessão Modo Console (TV, HDR, Auto-Login, Energia)"]
    end

    subgraph Service["Camada de Coordenação (launcher-app)"]
        LauncherService["LauncherService (Gestão de Estado & Ciclo de Vida)"]
    end

    subgraph CoreCrates["Crates Especializadas em Rust"]
        Core["launcher-core (Scanner, Headers, Discos, Plataformas)"]
        Steam["launcher-steam (VDF shortcuts, Grid artes, CDP WebSocket)"]
        Achiev["launcher-achievements (Relay de Conquistas & Activity Journal)"]
        DualSense["launcher-dualsense (Gatilhos Adaptáveis, RGB, UDP :6969, Emulação)"]
        Console["launcher-console (Display TV, Áudio, Shell Kiosk, Supervisor)"]
    end

    subgraph External["Subsistemas Externos"]
        SteamClient["Steam Client (shortcuts.vdf & CEF Port 8080)"]
        DualSenseHardware["Sony DualSense / Edge (USB / Bluetooth HID)"]
        GameMods["Mods de Telemetria de Jogos (Link UDP :6969)"]
        OS["Sistema Operacional (Win32 APIs / Linux xrandr / ALSA)"]
    end

    UI <--> LauncherService
    LauncherService --> Core
    LauncherService --> Steam
    LauncherService --> Achiev
    LauncherService --> DualSense
    LauncherService --> Console

    Steam <-->|"WebSocket CDP"| SteamClient
    Achiev -->|"Relay Notificação"| Steam
    DualSense <-->|"Raw HID 0x02/0x31"| DualSenseHardware
    DualSense <-->|"UDP JSON :6969"| GameMods
    Console <-->|"APIs de Sistema"| OS
```

---

## 2. Responsabilidades por Crate

### `launcher-core`
- **Catálogo de Plataformas:** Mapeamento imutável de mais de 30 plataformas de jogos, IDs de metadados externos (IGDB, RetroAchievements, libretro) e extensões de arquivos suportadas.
- **Leitor de Cabeçalhos de Cartuchos (`headers.rs`):** Inspeção com zero alocação de ROMs de GBA, NDS, Game Boy/Color, Nintendo 64 (com rotação de endianness), Sega Genesis e SNES (complement check).
- **Leitor de Imagens de Disco (`disc.rs`):** Parser ISO9660 PVD de baixo nível e leitor de descritores de volume, extraindo `SYSTEM.CNF` e `PARAM.SFO`.
- **Inspeção de Executáveis PE (`pe.rs`):** Recuperação de `ProductName` e busca de evidências locais de AppID da Steam (`steam_appid.txt`, `steam_emu.ini`, manifestos).
- **Scanner Multi-thread (`scanner.rs`):** Varredura concorrente em múltiplos discos através de `rayon` e `walkdir`.

### `launcher-steam`
- **Manipulador Binário de VDF (`vdf.rs`):** Parser recursivo do formato Valve Data Format binário e encoder atômico para `shortcuts.vdf`.
- **Cálculo de Shortcut AppID:** Hash determinístico de 32 bits:
  ```text
  shortcut_id = (CRC32(exe + AppName) | 0x80000000)
  ```
- **Gerenciador de Artes do Grid (`grid.rs`):** Sincronização de capas, heroes, logos, imagens wide e ícones na pasta da Steam.
- **Bridge CDP WebSocket (`cdp.rs`):** Conexão assíncrona ao Chrome DevTools Protocol da Steam em `127.0.0.1:8080` com anexo ao `SharedJSContext` para envio de notificações nativas de conquistas.

### `launcher-achievements`
- **Monitor de Conquistas (`manager.rs`):** Modelo de dados unificado para troféus e eventos de desbloqueio.
- **Histórico Persistente:** Gravação atômica em `activity.json`.
- **Parsers de Log (`models.rs`):** Reconhecimento de eventos de desbloqueio em logs de emuladores (como RPCS3) e encaminhamento ao bridge da Steam.

### `launcher-dualsense`
- **Comunicação HID Direta em Rust Puro (`protocol.rs`):** Protocolo Sony DualSense e DualSense Edge via USB (Output Report 0x02) e Bluetooth (Output Report 0x31 com checksum CRC32 IEEE 802.3 calculado via `crc32fast`).
- **Motor de Gatilhos Adaptáveis (`triggers.rs`):** Compilação de efeitos nos buffers de hardware oficiais de 11 bytes para L2 e R2: Resistência rígida contínua, Disparo mecânico de arma/semi-automático, Tensão de arco, Recuo de metralhadora, Galope, Pulsos de vibração e curvas customizadas de 10 zonas.
- **Barra de Luz RGB & LEDs de Jogador (`lightbar.rs`):** Efeitos estáticos, pulsação suave, ciclo arco-íris e indicador dinâmico de nível de bateria, além de LEDs indicadores de Jogador 1 a 4 e LED de microfone.
- **Servidor Link UDP DualSenseX (`udp_server.rs`):** Servidor assíncrono em `tokio::net::UdpSocket` na porta `6969`, decodificando pacotes JSON enviados por mods de jogos (Assetto Corsa, F1 23/24, Cyberpunk 2077, etc.).
- **Emulação de Controles & Giroscópio (`emulation.rs`):** Perfis de controle virtual (DualSense Nativo, Xbox 360 XInput via ViGEmBus/uinput, DualShock 4) e mapeamento do giroscópio IMU de 6 eixos para mira precisa no analógico direito.
- **Perfis de Jogos (`profiles.rs`):** Configurações pré-definidas para Simuladores de Corrida (pulso ABS no freio L2 e resistência de acelerador no R2), Jogadores Táticos de Tiro, Simuladores de Voo e Modo Clássico.

### `launcher-console`
- **Supervisor de Janelas (`supervisor.rs`):** Gerencia os dois comportamentos de execução:
  1. `HideAndRestoreFullscreen`: Oculta o launcher durante a execução do jogo e o restaura com foco imediato no controle ao fechar.
  2. `KeepOpen`: Mantém o launcher aberto em segundo plano.
- **Configurações de TV (`models.rs`):** Presets para 4K a 120Hz, 1440p a 120Hz, 1080p a 120Hz e ativação de HDR.
- **Inicialização do Sistema (`boot.rs`):** Automação de auto-login (`AutoAdminLogon` no Windows / desktop autostart no Linux).
- **Ações de Energia (`power.rs`):** Comandos nativos para Suspender, Hibernar, Reiniciar e Desligar via controle.

### `launcher-app`
- **Serviço Central (`service.rs`):** `LauncherService` agregando estado, persistência de catálogo, configurações e chamadas IPC.
- **Mecanismo de Internacionalização (`i18n.rs`):** Dicionário bilíngue com padrão em inglês (`en`) e suporte integral a português (`pt-BR`).
- **Comandos e Flags de CLI (`cli.rs`):** Parser de linha de comando com suporte a `--lang`, `--set-lang`, `status`, `scan`, `export-steam` e `set-mode`.
- **Binário Executável (`main.rs`):** Ponto de entrada com logging estruturado via `tracing-subscriber`.

