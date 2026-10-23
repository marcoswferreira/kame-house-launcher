# 🏝️ Kame House Launcher - Identidade Visual e Branding

> *"Sua Sala. Seu Refúgio. Seus Jogos."*

---

## 1. Filosofia de Marca e Resonância Emocional

### O Conceito: O Refúgio do Gamer
No universo de Dragon Ball, a **Kame House** nunca foi um campo de batalha — ela é uma ilha pacífica, isolada no meio do oceano sob um céu ensolarado com coqueiros. É o lugar para onde os guerreiros voltam exaustos para descansar, comer, treinar e celebrar com seus amigos antes da próxima jornada.

Transformar o PC da sala em console muitas vezes parece uma luta constante contra o sistema operacional: popups do Windows, ícones bagunçados na área de trabalho, janelas frias e lojas corporativas cinzentas.

O **Kame House Launcher** nasceu para trazer essa sensação de refúgio aconchegante para a sala de estar:
- **Zero Fricção de Desktop:** Ao ligar o computador na TV, a área de trabalho desaparece por completo.
- **Calor em vez de Agressividade:** Substitui o visual genérico "gamer cyberpunk neon" por um pôr do sol dourado, tons suaves de rosa coral e o azul profundo do oceano.
- **A Nuvem Voadora (Kinto'un):** Construído 100% em Rust puro, o launcher é ultrarrápido e consome menos de 40MB de RAM. Assim como a Nuvem Voadora só aceita aqueles de coração puro, nosso launcher é focado exclusivamente na pureza da experiência de jogar: sem telemetria pesada, sem anúncios e sem lentidão.

---

## 2. Paleta de Cores e Tokens de Design

A identidade visual é ancorada nas cores tropicais ao entardecer da Kame House:

```
+-------------------------------------------------------------------------------+
|  Kame Coral     Nimbus Gold    Terra Cotta    Pacific Midnight   Sunset Cream |
|    #FB7185        #FBBF24        #EA580C          #0A1128          #FFFBEB    |
| (Rosa Aconchego)(Foco/Nuvem)  (Telhado Telha)   (Base OLED)     (Texto Claro) |
+-------------------------------------------------------------------------------+
```

### Especificação de Cores

| Nome do Token | Hex Code | RGB | Função e Aplicação |
| :--- | :--- | :--- | :--- |
| `--color-bg-deep` | `#0A1128` | `rgb(10, 17, 40)` | Fundo oceânico profundo. Preto perfeito em TVs OLED sem causar cansaço visual. |
| `--color-surface` | `#1E293B` | `rgb(30, 41, 59)` | Cards de jogos, modais flutuantes e painéis com borda semitransparente de 1px. |
| `--color-kame-coral` | `#FB7185` | `rgb(251, 113, 133)` | Marca registrada. Tom das paredes da casa, botões primários e destaques visuais. |
| `--color-nimbus-gold`| `#FBBF24` | `rgb(251, 191, 36)` | Anel de foco do controle (D-pad), desbloqueio de troféus, guias ativas e brilho. |
| `--color-roof-red` | `#EA580C` | `rgb(234, 88, 12)` | Vermelho terracota do telhado. Badges de destaque, botões de saída e avisos. |
| `--color-palm-green` | `#10B981` | `rgb(16, 185, 129)` | Status ativo do Sunshine, bateria cheia do joystick, conquistas platinadas. |
| `--color-text-main` | `#FFFBEB` | `rgb(255, 251, 235)` | Branco quente das nuvens para leitura clara e agradável a 3 metros da TV. |
| `--color-text-muted`| `#94A3B8` | `rgb(148, 163, 184)` | Metadados secundários (plataformas, tempo de jogo, desenvolvedora). |

---

## 3. Hierarquia Tipográfica (Otimizada para 10-Foot Console)

Criada sob medida para televisores 4K de 55" a 85":

```
TÍTULOS E HERO   : Outfit / Plus Jakarta Sans (ExtraBold 900)
NAVEGAÇÃO DA UI  : Plus Jakarta Sans / Inter (SemiBold 600)
TEXTO E METADADOS: Inter / Segoe UI (Regular 400)
DADOS E HUD RETRÔ: JetBrains Mono / Fira Code (Medium 500)
```

- **Distância do Sofá:** Todos os títulos de cartuchos e jogos têm altura mínima renderizada de `28px`, assegurando legibilidade sem esforço a partir de 3 metros de distância.
- **Espaçamento:** Títulos possuem tracking levemente expandido (`+1.5px`) para evitar embolamento de caracteres em painéis HDR de alta luminosidade.

---

## 4. Metáforas da Interface (Anatomia do Console)

A experiência do launcher é organizada em 4 áreas conceituais:

```
+-------------------------------------------------------------------------------+
|  A VARANDA (Carrossel Inicial) -> Jogos recentes com hero banners dinâmicos  |
|  A ILHA DE TREINO (Catálogo)   -> Todas as 30+ plataformas, ROMs e jogos de PC|
|  O FAROL (Painel Sunshine)     -> Streaming Moonlight, pareamento PIN e rede |
|  A OFICINA (Modo Console)      -> Resoluções de TV 4K 120Hz, boot e energia   |
+-------------------------------------------------------------------------------+
```

1. **A Varanda (Carrossel de Entrada):**
   - Ao focar em um jogo, o horizonte oceânico escurece suavemente e dá lugar ao banner cinematográfico (`_hero.png`) e logo transparente (`_logo.png`) do jogo a 60/120 FPS.
2. **O Anel de Foco da Nuvem Voadora:**
   - Ao navegar com as setas do controle, o elemento selecionado recebe uma aura dourada suave e borda amarela de 2px (`#FBBF24`), dispensando qualquer uso de mouse.
3. **Notificação de Conquistas (Rastro Dourado):**
   - Ao conquistar um troféu em emuladores ou lojas, um rastro dourado com som clássico de sino surge no canto superior, integrando-se com a notificação nativa da Steam.

---

## 5. Paisagem Sonora e Resposta Tátil

- **Clique de Navegação:** Toque acústico macio de madeira seca (inspirado na varanda de madeira da casa).
- **Entrada no Jogo:** Sopro suave de vento que evolui para um sino cristalino (a decolagem da Nuvem Voadora).
- **Vibração Tátil (Haptics):** Vibração sutil nos motores de feedback do controle ao deslizar pelas capas dos jogos.

---

## 6. Tom de Voz da Marca e Slogans

- **Personalidade:** Acolhedor, nostálgico, incrivelmente veloz, moderno e apaixonado pela preservação de jogos.
- **Slogan Principal (PT-BR):** *"Sua Sala. Seu Refúgio. Seus Jogos."*
- **Slogan da Comunidade:** *"Esqueça o desktop. Seja bem-vindo ao seu refúgio."*
