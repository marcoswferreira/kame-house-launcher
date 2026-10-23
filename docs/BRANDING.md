# 🏝️ Kame House Launcher - Brand Identity & Design System

> *"Your Living Room. Your Sanctuary. Your Games."*

---

## 1. Brand Philosophy & Emotional Core

### The Concept: The Gamer's Sanctuary
In the Dragon Ball universe, **Kame House** is not a battlefield—it is an isolated sanctuary surrounded by endless ocean, sunny skies, and palm trees. It is the place where warriors lay down their weights, relax, share meals, and bond before the next journey.

Modern PC gaming in the living room often feels like fighting the operating system: desktop popups, taskbar clutter, keyboard prompts, and cold corporate storefronts.

**Kame House Launcher** transforms your living room into that secluded island sanctuary:
- **Zero OS Friction:** The moment your PC boots, the Windows/Linux desktop vanishes.
- **Warmth Over Aggression:** Replacing aggressive "cyberpunk gamer neon" with warm golden-hour sunsets, coral pinks, and deep oceanic blues.
- **The Flying Nimbus (Kinto'un):** Built entirely in pure Rust, the launcher is featherweight and blazing fast. Like the Nimbus, it only responds to purity: pure gaming focus without telemetry, bloat, or ads.

---

## 2. Visual Palette & Design Tokens

The visual identity is anchored directly in the tropical sunset palette of Kame House:

```
+-------------------------------------------------------------------------------+
|  Kame Coral     Nimbus Gold    Terra Cotta    Pacific Midnight   Sunset Cream |
|    #FB7185        #FBBF24        #EA580C          #0A1128          #FFFBEB    |
| (Warm Island)  (Focus/Nimbus) (Roof Accent)     (OLED Base)       (Body Text) |
+-------------------------------------------------------------------------------+
```

### Color Specifications

| Token Name | Hex Code | RGB | Role / Usage |
| :--- | :--- | :--- | :--- |
| `--color-bg-deep` | `#0A1128` | `rgb(10, 17, 40)` | Deep ocean canvas. OLED pure black contrast without eye strain. |
| `--color-surface` | `#1E293B` | `rgb(30, 41, 59)` | Cards, dialogs, shelf containers with 1px semi-transparent border. |
| `--color-kame-coral` | `#FB7185` | `rgb(251, 113, 133)` | Brand hallmark. House wall tint, interactive buttons, vibrant accents. |
| `--color-nimbus-gold`| `#FBBF24` | `rgb(251, 191, 36)` | Controller focus rings, achievement unlocks, active tabs, glow aura. |
| `--color-roof-red` | `#EA580C` | `rgb(234, 88, 12)` | Kame House terracotta roof. Badges, exit actions, urgent indicators. |
| `--color-palm-green` | `#10B981` | `rgb(16, 185, 129)` | Moonlight streaming active, controller battery full, achievements done. |
| `--color-text-main` | `#FFFBEB` | `rgb(255, 251, 235)` | Warm cloud white text for 10-foot readability from 3 meters away. |
| `--color-text-muted`| `#94A3B8` | `rgb(148, 163, 184)` | Secondary metadata (platform tags, playtime, developer info). |

---

## 3. Typography Hierarchy (10-Foot Display Optimized)

Designed specifically for readability across 4K 55"–85" television screens:

```
DISPLAY HERO     : Outfit / Plus Jakarta Sans (ExtraBold 900)
UI NAVIGATION    : Plus Jakarta Sans / Inter (SemiBold 600)
BODY & METADATA  : Inter / Segoe UI (Regular 400)
RETRO HUD / STATS: JetBrains Mono / Fira Code (Medium 500)
```

- **Screen Distances:** All interactive card titles have a minimum rendered height of `28px` to guarantee legibility at standard couch distance ($3\text{m} / 10\text{ft}$).
- **Letter Spacing:** Headlines utilize slight tracking (`+1.5px`) to prevent visual crowding under HDR brightness.

---

## 4. UI Metaphors & Anatomy

The launcher interface is divided into 4 thematic spaces:

```
+-------------------------------------------------------------------------------+
|  THE PORCH (Home Shelf)        -> Recent games with dynamic wide hero banners |
|  THE TRAINING GROUNDS (Catalog)-> All platforms, ROMs, and PC titles         |
|  THE LOOKOUT (Sunshine Hub)    -> Moonlight stream control & PIN pairing      |
|  THE WORKSHOP (Console Mode)   -> 4K 120Hz TV presets, autostart, power menu  |
+-------------------------------------------------------------------------------+
```

1. **The Porch (Home Shelf):**
   - Highlighting a game dims the ocean horizon and illuminates the custom hero banner (`_hero.png`) and transparent logo (`_logo.png`) in 60 FPS motion.
2. **The Controller Focus Ring (Nimbus Trail):**
   - When moving with the D-pad or analog stick, the active element is enveloped in a subtle golden Nimbus aura with a 2px glowing amber outline (`#FBBF24`).
3. **Achievement Notifications (Nimbus Streak):**
   - When a trophy or RetroAchievement pops, a golden streak sweeps across the top-right corner of the screen, revealing the official Steam toast sound and badge.

---

## 5. Soundscape & Haptic Tone

- **Navigation Click:** Warm, muted wooden acoustic resonance (inspired by the wooden porch of Kame House).
- **Game Launch:** Subtle atmospheric wind rushing into an energetic chime (The Nimbus taking flight).
- **Gamepad Rumble:** Gentle dual-motor pulse upon cycling between game cards, giving tactile weight to cover art.

---

## 6. Brand Voice & Taglines

- **Tone:** Welcoming, nostalgic, uncompromisingly fast, modern, and respectful of video game preservation.
- **Tagline (EN):** *"Your Living Room. Your Sanctuary. Your Games."*
- **Community Tagline:** *"Leave the desktop behind. Welcome home."*
