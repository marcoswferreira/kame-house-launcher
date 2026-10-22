# Guia do Modo Console: Transformando seu PC em um Console de Sala 📺🎮

O **Modo Console** do Kame House Launcher foi projetado para transformar qualquer computador comum em um console doméstico dedicado, eliminando todas as fricções típicas de um sistema operacional de desktop de escritório.

---

## 1. Inicialização Direta (Cold Boot)

Para que a experiência seja idêntica à de um console, a máquina deve ligar diretamente na interface do launcher, sem telas de senha ou visibilidade do desktop.

### 1.1 Auto-Login sem Senha
- **No Windows:**
  O módulo `launcher-console` gera o arquivo de registro com as chaves oficiais de `AutoAdminLogon`:
  ```reg
  [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon]
  "AutoAdminLogon"="1"
  "DefaultUserName"="SeuUsuario"
  "DefaultDomainName"="."
  ```
- **No Linux:**
  Configuração automatizada através de `greetd`, `gdm` ou `lightdm` com `autologin-user=<usuario>`.

### 1.2 Inicialização Automática com o Sistema
- **Linux:** Gera automaticamente a entrada `kame-house-launcher.desktop` no diretório `~/.config/autostart/` executando com a flag `--fullscreen`.
- **Windows:** Registro em `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` com inicialização em modo maximizado.

### 1.3 Modo Kiosk / Shell Alternativo
No Windows, é possível definir o Kame House Launcher como shell primário em substituição ao `explorer.exe` (ou executá-lo imediatamente antes), garantindo que a barra de tarefas e ícones da área de trabalho não fiquem visíveis.

---

## 2. Ajustes de Display e Áudio para Sala de Estar

### 2.1 Presets de TV em Alta Taxa de Atualização
Ao conectar o PC a uma televisão moderna via HDMI 2.1, o launcher oferece seletores rápidos:
- **4K @ 120Hz:** Resolução 3840x2160 com taxa de atualização de 120Hz para TVs OLED / QLED.
- **1440p @ 120Hz:** Excelente equilíbrio entre resolução e taxa de quadros para placas de vídeo intermediárias.
- **1080p @ 120Hz / 60Hz:** Padrão para telas mais antigas.

### 2.2 Ativação e Calibração de HDR
O Modo Console permite habilitar o High Dynamic Range (HDR) com um clique e invoca o assistente de calibração HDR do sistema para cores vívidas e pretos profundos.

### 2.3 Roteador Automático de Áudio
Garante que o som dos jogos seja direcionado para a saída correta (HDMI da TV, Soundbar via HDMI eARC ou fone conectado ao controle sem fio), evitando que o áudio permaneça preso nas caixas de som do escritório.

---

## 3. Comportamento de Inicialização de Jogos (Launch Behavior)

O launcher disponibiliza **dois comportamentos de execução** totalmente configuráveis:

### Modo 1: Ocultar e Restaurar em Tela Cheia (**Padrão do Console**)
- **Ao iniciar o jogo:** O launcher oculta ou minimiza sua janela, liberando 100% dos recursos de GPU e composição de janela para o jogo.
- **Durante o jogo:** O supervisor de processos monitora em segundo plano a execução do executável ou emulador.
- **Ao fechar o jogo:** O launcher restaura imediatamente sua janela em **tela cheia** e retoma o foco do controle instantaneamente no menu de jogos. Não há retorno à área de trabalho do Windows.

### Modo 2: Manter Aberto em Segundo Plano
- Mantém o launcher aberto em segundo plano enquanto o jogo é executado, útil para máquinas com múltiplos monitores onde o jogador deseja manter o catálogo visível em uma segunda tela.

---

## 4. Navegação 10-Pés & Gamepad

- **Navegação Integral via Controle:** Compatível nativamente com controles de Xbox (XInput), PlayStation DualShock 4 / DualSense e Nintendo Switch Pro.
- **Ocultação Automática do Cursor:** Assim que qualquer botão do controle é pressionado, o cursor do mouse é ocultado da tela.
- **Teclado Virtual em Tela (OSK):** Permite pesquisar títulos, digitar termos e senhas Wi-Fi usando apenas o D-pad e os botões do controle.
- **Design Sonoro de Console:** Feedback sonoro discreto a cada movimento e clique, trazendo a sensação tátil de um console moderno.

---

## 5. Menu de Energia e Wake-on-LAN

### 5.1 Ações de Energia pelo Controle
Pressionando o botão de guia (Home / Xbox / PS) ou selecionando o ícone de energia:
- **Suspender:** Coloca o PC em modo de espera com baixo consumo de energia.
- **Hibernar:** Salva o estado em disco e desliga.
- **Reiniciar:** Reinicialização completa da máquina.
- **Desligar:** Desligamento seguro do sistema operacional.

### 5.2 Wake-on-LAN (WoL) para Moonlight
Permite configurar e validar a placa de rede do computador para permitir que clientes Moonlight em televisores, celulares ou portáteis acordem o computador remotamente pela rede local (Magic Packet).
