# ADR 0001: Arquitetura do Launcher em Rust e Tauri v2

- **Status:** Proposto (Aceito para Planejamento)
- **Data:** 2026-10-22
- **Decisores:** Time de Desenvolvimento / Engenharia

## Contexto e Declaração do Problema

O Kame House Launcher foi concebido como uma central de jogos de alto desempenho para transformar computadores comuns em consoles de sala de estar (Living Room Console PC).

Os requisitos técnicos fundamentais são:
1. Uma base de código sólida em **Rust**, com arquitetura modular, tipagem estrita, performance nativa e segurança de memória.
2. Alta performance em operações de baixo nível: varredura e hashing multi-thread de milhares de ROMs/ISOs, parsing de binários proprietários (Valve VDF, ISO9660, PARAM.SFO, PE executáveis), inspeção de discos e chamadas ao sistema operacional com footprint de memória residual ínfimo (<60MB de RAM).
3. Uma interface de usuário de 10 pés ("10-foot UI") fluida a 60/120fps, com navegação total via gamepad/controle, feedback sonoro e renderização de artes visuais em alta resolução.
4. Fácil integração e incorporação do estúdio DualSense e da sessão de configuração "Console Mode".

## Opções Consideradas

1. **Opção 1: Workspace Rust Modular + Tauri v2 (Recomendada)**
   - Backend 100% Rust distribuído em crates especializadas.
   - Frontend em WebView nativo (WebKit/WebView2 via Tauri v2), aproveitando tecnologias web modernas (TypeScript/Tailwind) para a interface de 10 pés, carrosséis de jogos, layout de console e painéis de controle.
   - Pegada de memória de ~40MB a 60MB (redução drástica comparada a navegadores pesados empacotados).
   - Suporte nativo a aceleração de hardware e gamepad.

2. **Opção 2: Pure Rust GUI (Slint ou Iced)**
   - Renderização gráfica nativa direta via Skia ou wgpu.
   - Excelente consumo de memória (<30MB) e tempo de compilação compacto.
   - *Desvantagens:* Menor flexibilidade para renderizar componentes dinâmicos de telemetria e calibração de controles, maior esforço para layouts complexos de carrossel de jogos, animações e integração com o ecossistema.

## Decisão

Adotamos a **Opção 1**: Um workspace modular em **Rust** combinado com **Tauri v2** para a camada de apresentação.

### Estrutura Modular das Crates no Workspace Cargo

```text
kame-house-launcher/
├── Cargo.toml (Workspace raiz)
├── frontend/                  # Interface React 19 + TypeScript embutida no launcher-app
└── crates/
    ├── launcher-core/         # Tipos centrais, biblioteca de jogos, scanner, identificação de ROMs/PE
    ├── launcher-steam/        # Injeção em shortcuts.vdf, grid de artes e bridge CDP WebSocket para Steam CEF
    ├── launcher-achievements/ # Monitor de conquistas e relay de notificações
    ├── launcher-dualsense/    # Extensão opt-in DualSense HID, gatilhos adaptáveis, RGB e UDP :6969
    ├── launcher-console/      # Roteamento de mudo de áudio/microfone e supervisor de lançamento
    └── launcher-app/          # LauncherService central, servidor HTTP, System Tray e CLI
```

## Consequências

### Positivas
- **Eficiência e Velocidade:** As varreduras de arquivos, hashes criptográficos e injeções VDF rodam com velocidade nativa multi-thread (Rayon/Tokio).
- **Consumo Mínimo de Recursos:** O launcher pode permanecer ativo na bandeja do sistema consumindo apenas dezenas de megabytes enquanto o jogo ou streaming estiver em execução.
- **Console Experience:** WebViews modernas facilitam a criação de transições suaves estilo PS5/Xbox Series, temas escuros para TV e suporte completo a controles com biblioteca Gamepad API padronizada.
- **Portabilidade:** Compatibilidade de compilação direta para Windows e Linux.

### Negativas / Mitigações
- **Dependência do WebView do Sistema:** No Windows usa WebView2 (pré-instalado no Win10/11); no Linux usa WebKitGTK. A instalação em ambientes Linux customizados necessita dos pacotes WebKitGTK básicos.

