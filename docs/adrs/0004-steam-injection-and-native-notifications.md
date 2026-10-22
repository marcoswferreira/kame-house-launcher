# ADR 0004: Interoperabilidade com a Steam, Injeção VDF e Notificações Nativas de Conquistas

- **Status:** Proposto (Aceito para Planejamento)
- **Data:** 2026-10-22
- **Decisores:** Time de Desenvolvimento / Engenharia

## Contexto e Declaração do Problema

Para integrar perfeitamente os jogos de emuladores e de lojas de PC ao ecossistema da Steam nativa em ambientes de sala de estar:
1. Adicionar atalhos em `shortcuts.vdf` sem corromper a estrutura binária da Valve.
2. Posicionar artes personalizadas na pasta `userdata/<account>/config/grid`.
3. Injetar scripts de visualização na interface CEF da Steam e disparar notificações nativas de conquista (com som e animação oficiais da Steam) quando o jogador desbloqueia uma conquista em emuladores (RetroAchievements, RPCS3, Vita3K) ou lojas (Epic, GOG, Xbox).

Implementamos toda essa infraestrutura em Rust com máxima confiabilidade, sem depender de frameworks externos pesados como SFP ou Millennium.

## Decisão

Implementamos essa funcionalidade dividida em duas crates especializadas:
- `launcher-steam`: Parser/Encoder binário de VDF e Injetor CDP WebSocket.
- `launcher-achievements`: Observador e gerenciador de fontes de conquistas com relay para a Steam.

### 1. Manipulador Binário de VDF (`shortcuts.vdf`)
Implementado em Rust puro:
- Parser recursivo para o formato Valve Data Format binário (tipos: `0x00` objeto, `0x01` string null-terminated, `0x02` int32 LE, `0x03` float32, `0x06` int64, `0x08` terminador).
- Cálculo idêntico ao da Steam para o Shortcut AppID de 32 bits:
  $$\text{shortcut\_id} = (\text{CRC32}(\text{exe} + \text{AppName}) \mid 0x80000000) \pmod{2^{32}}$$
- Gravação atômica (`tempfile` + `rename`) com backup automático para proteger contra interrupções abruptas de energia.
- Sincronização automática das artes para o diretório `userdata/<SteamID>/config/grid/`:
  - Capa: `<shortcut_id>p.png`
  - Hero: `<shortcut_id>_hero.png`
  - Logo: `<shortcut_id>_logo.png`
  - Wide: `<shortcut_id>.png`
  - Ícone: `<shortcut_id>_icon.ico`

### 2. Injetor CDP WebSocket (Chrome DevTools Protocol)
- A Steam é iniciada com os argumentos `-cef-enable-debugging -devtools-port 8080`.
- O launcher conecta-se via WebSocket em `ws://127.0.0.1:8080/devtools/browser/...` utilizando `tokio-tungstenite`.
- Descobre janelas via `Target.setDiscoverTargets` e anexa sessões planas (`Target.attachToTarget`):
  - **`SharedJSContext`:** Contexto central de background da Steam. Nele é injetado o listener de notificações que invoca as APIs nativas do cliente Steam para disparar o toast oficial de conquista com som.
  - **`Steam` e `Steam Big Picture Mode`:** Contextos de exibição da biblioteca. Permite renderizar detalhes ricos do jogo (descrições, botões de ação e dados da comunidade).

### 3. Gerenciador de Conquistas Unificado (`launcher-achievements`)
- Observadores não intrusivos de logs e bancos de dados locais:
  - **RetroAchievements:** Consulta via API RA / watcher de log do RetroArch ou emuladores suportados.
  - **RPCS3:** Leitura do banco SQLite/trophy data local do emulador de PS3.
  - **Vita3K / shadPS4:** Watchers de troféus exportados.
  - **Stores (Epic Games, GOG Galaxy, Xbox):** APIs oficiais / autenticação com tokens salvos em cofre seguro local.
- Quando uma conquista é desbloqueada, o evento é emitido:
  1. Registrado no histórico local de atividades (`activity.json` ou SQLite).
  2. Encaminhado imediatamente via WebSocket CDP para o `SharedJSContext` da Steam, apresentando a notificação nativa ao jogador.

## Consequências

### Positivas
- **Imersão Total:** O usuário joga um título de PS3 no RPCS3 ou um clássico de SNES e vê a notificação oficial da Steam surgir no canto da tela, exatamente como se fosse um jogo nativo da Valve.
- **Segurança e Estabilidade:** Sem DLL injection, sem hooks de memória instáveis ou modificações nos binários da Steam; usa a API oficial de depuração Chromium que a própria Valve disponibiliza.
- **Independência:** Funciona tanto no modo desktop comum quanto no Steam Big Picture Mode.

