import React, { useState } from 'react';
import { useTranslation } from '../../../i18n';
import { ExternalLink, Search, Sparkles, Tv, Gamepad, Cpu } from 'lucide-react';

interface UtilityItem {
  id: string;
  name: string;
  category: 'streaming' | 'emulators' | 'tools';
  categoryLabel: { en: string; pt: string };
  tag: string;
  url: string;
  isSteamStore?: boolean;
  description: {
    en: string;
    pt: string;
  };
  kameTip: {
    en: string;
    pt: string;
  };
}

const UTILITIES_DATA: UtilityItem[] = [
  {
    id: 'sunshine',
    name: 'Sunshine',
    category: 'streaming',
    categoryLabel: { en: 'Remote Streaming', pt: 'Streaming Remoto' },
    tag: 'Host Server',
    url: 'https://github.com/LizardByte/Sunshine',
    description: {
      en: 'High-performance, low-latency open-source game stream server. Transmits your PC desktop and games directly to TVs, mobile devices, and handhelds.',
      pt: 'Servidor de streaming de jogos open-source de altíssimo desempenho e baixa latência. Transmite seus jogos do PC para a TV da sala, celular ou Steam Deck.',
    },
    kameTip: {
      en: 'Run Sunshine as a service on this PC so you can wake and stream directly into Moonlight on your TV without keyboard/mouse interaction.',
      pt: 'Execute o Sunshine como serviço no PC para transmitir diretamente para o Moonlight na TV sem precisar de teclado ou mouse.',
    },
  },
  {
    id: 'moonlight',
    name: 'Moonlight Game Streaming',
    category: 'streaming',
    categoryLabel: { en: 'Remote Streaming', pt: 'Streaming Remoto' },
    tag: 'Client Receiver',
    url: 'https://moonlight-stream.org/',
    description: {
      en: 'Open-source client implementation of NVIDIA GameStream/Sunshine. Streams games up to 4K @ 120 FPS with HDR and full gamepad vibration.',
      pt: 'Cliente de streaming de código aberto compatível com Sunshine. Recebe a transmissão em até 4K a 120 FPS com HDR e vibração total no controle.',
    },
    kameTip: {
      en: 'Install Moonlight on your Living Room Android TV, Apple TV, or laptop to turn any display into a living room console.',
      pt: 'Instale o Moonlight na sua Smart TV Android, Apple TV ou TV Box para transformar qualquer tela na sua sala em um console.',
    },
  },
  {
    id: 'rpcs3',
    name: 'RPCS3',
    category: 'emulators',
    categoryLabel: { en: 'Sony PlayStation 3', pt: 'Sony PlayStation 3' },
    tag: 'PS3 Emulator',
    url: 'https://rpcs3.net/',
    description: {
      en: 'The world leading open-source Sony PlayStation 3 emulator and debugger written in C++ for Windows and Linux.',
      pt: 'O principal emulador de PlayStation 3 do mundo para PC, capaz de rodar grandes clássicos do PS3 com resolução 4K e 60 FPS.',
    },
    kameTip: {
      en: 'Kame House automatically parses RPCS3 logs to trigger official Steam achievement toast overlays with audio in real time.',
      pt: 'O Kame House monitora os logs do RPCS3 para disparar as notificações oficiais de conquistas da Steam com som e arte na sua tela.',
    },
  },
  {
    id: 'xenia',
    name: 'Xenia',
    category: 'emulators',
    categoryLabel: { en: 'Microsoft Xbox 360', pt: 'Microsoft Xbox 360' },
    tag: 'Xbox 360 Emulator',
    url: 'https://xenia.jp/',
    description: {
      en: 'Open-source research emulator for the Microsoft Xbox 360 console, supporting modern Vulkan and DirectX 12 backends.',
      pt: 'Emulador open-source de ponta para Microsoft Xbox 360, focado em alta compatibilidade gráfica e suporte a Vulkan e DirectX 12.',
    },
    kameTip: {
      en: 'Place your Xbox 360 ROMs/ISOs in your configured scan directories; Kame House will index and link them into Steam Big Picture.',
      pt: 'Coloque seus jogos de Xbox 360 nas pastas cadastradas do Kame House para sincronizar os atalhos diretamente na Steam.',
    },
  },
  {
    id: 'pcsx2',
    name: 'PCSX2',
    category: 'emulators',
    categoryLabel: { en: 'Sony PlayStation 2', pt: 'Sony PlayStation 2' },
    tag: 'PS2 Emulator',
    url: 'https://pcsx2.net/',
    description: {
      en: 'Leading open-source PlayStation 2 emulator featuring advanced widescreen patches, texture replacement, and 4K internal upscaling.',
      pt: 'O mais avançado emulador de PlayStation 2, com suporte a upscaling para 4K, patches de tela widescreen 16:9 e melhorias visuais.',
    },
    kameTip: {
      en: 'Kame House identifies PS2 ISOs by inspecting SYSTEM.CNF sectors and synchronizes them directly into shortcuts.vdf.',
      pt: 'O scanner do Kame House inspeciona o setor SYSTEM.CNF das ISOs de PS2 para identificar os jogos e gerar atalhos perfeitos na Steam.',
    },
  },
  {
    id: 'duckstation',
    name: 'DuckStation',
    category: 'emulators',
    categoryLabel: { en: 'Sony PlayStation 1', pt: 'Sony PlayStation 1' },
    tag: 'PS1 Emulator',
    url: 'https://www.duckstation.org/',
    description: {
      en: 'Focuses on playability, speed, and long-term maintainability for Sony PlayStation 1, with PGXP geometry correction.',
      pt: 'Emulador de PlayStation 1 focado em precisão e melhorias gráficas, eliminando as deformações de textura 3D originais com PGXP.',
    },
    kameTip: {
      en: 'Pairs great with Kame House automatic disc inspection for .bin, .cue, and .chd PlayStation format archives.',
      pt: 'Totalmente compatível com as imagens .iso, .cue e .chd indexadas pelo Kame House com capas automáticas no grid da Steam.',
    },
  },
  {
    id: 'dolphin',
    name: 'Dolphin',
    category: 'emulators',
    categoryLabel: { en: 'GameCube & Wii', pt: 'GameCube & Wii' },
    tag: 'GC & Wii Emulator',
    url: 'https://dolphin-emu.org/',
    description: {
      en: 'Emulator for two Nintendo video game consoles: GameCube and Wii. Enables 1080p/4K resolution, enhancements, and savestates.',
      pt: 'Emulador aclamado para GameCube e Nintendo Wii, permitindo jogar em 4K a 60 FPS com suporte a controles modernos e save states.',
    },
    kameTip: {
      en: 'Use the DualSense Studio tab to configure authentic GameCube trigger click barrier emulation for racing games.',
      pt: 'Use a aba DualSense Studio do Kame House para emular a barreira mecânica de clique do gatilho analógico do GameCube.',
    },
  },
  {
    id: 'retroarch',
    name: 'RetroArch',
    category: 'emulators',
    categoryLabel: { en: 'Multi-System Frontend', pt: 'Multi-Sistemas' },
    tag: 'All-in-One Engine',
    url: 'https://www.retroarch.com/',
    description: {
      en: 'Frontend for emulators, game engines, and media players. Supports GBA, SNES, Genesis, NES, N64, and RetroAchievements.',
      pt: 'Frontend universal para emuladores e motores de jogos. Reúne núcleos para GBA, SNES, Mega Drive, N64 e suporte ao RetroAchievements.',
    },
    kameTip: {
      en: 'Kame House scans all your cartridge headers and maps them directly into Steam with categoric tags for each console generation.',
      pt: 'O Kame House analisa o cabeçalho dos cartuchos e os adiciona à Steam com tags categorizadas por console.',
    },
  },
  {
    id: 'lossless-scaling',
    name: 'Lossless Scaling',
    category: 'tools',
    categoryLabel: { en: 'Gaming Optimization', pt: 'Otimização de Jogos' },
    tag: 'Frame Generation',
    url: 'https://store.steampowered.com/app/993090/Lossless_Scaling/',
    isSteamStore: true,
    description: {
      en: 'Universal Frame Generation (LSFG) and spatial upscaling (LS1/FSR) for any PC game or emulator, doubling framerates smoothly.',
      pt: 'Geração universal de quadros (LSFG) e upscaling espacial (LS1/FSR) para qualquer jogo ou emulador, dobrando a fluidez na sua TV.',
    },
    kameTip: {
      en: 'Transform 30/60 FPS locked console titles or demanding PC games into buttery-smooth 60/120 FPS on your TV screen.',
      pt: 'Excelente para dobrar o framerate de jogos de emulador travados em 30 FPS para 60 FPS ou 120 FPS fluidos na TV da sala.',
    },
  },
];

interface Props {
  onShowToast?: (message: string) => void;
}

export const UtilitiesTab: React.FC<Props> = ({ onShowToast }) => {
  const { t, lang } = useTranslation();
  const [filter, setFilter] = useState<'all' | 'streaming' | 'emulators' | 'tools'>('all');
  const [search, setSearch] = useState('');

  const handleOpenUrl = async (url: string) => {
    try {
      const res = await fetch('/api/open-url', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ url }),
      });
      if (res.ok) {
        onShowToast?.(t.toast_url_opened);
      } else {
        window.open(url, '_blank', 'noopener,noreferrer');
      }
    } catch {
      window.open(url, '_blank', 'noopener,noreferrer');
    }
  };

  const filteredItems = UTILITIES_DATA.filter((item) => {
    const matchesFilter = filter === 'all' || item.category === filter;
    const nameMatch = item.name.toLowerCase().includes(search.toLowerCase());
    const descMatch = item.description[lang === 'pt' ? 'pt' : 'en']
      .toLowerCase()
      .includes(search.toLowerCase());
    return matchesFilter && (nameMatch || descMatch);
  });

  return (
    <section className="tab-panel">
      <div className="panel-header">
        <h1 className="panel-title">{t.utilities_title}</h1>
        <p className="panel-desc">{t.utilities_desc}</p>
      </div>

      {/* Filter and Search Controls */}
      <div
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
          flexWrap: 'wrap',
          gap: '12px',
          marginBottom: '20px',
        }}
      >
        <div style={{ display: 'flex', gap: '8px', flexWrap: 'wrap' }}>
          <button
            className={`subnav-pill ${filter === 'all' ? 'active' : ''}`}
            onClick={() => setFilter('all')}
          >
            <Sparkles size={14} style={{ marginRight: '6px' }} />
            {t.utilities_filter_all}
          </button>
          <button
            className={`subnav-pill ${filter === 'streaming' ? 'active' : ''}`}
            onClick={() => setFilter('streaming')}
          >
            <Tv size={14} style={{ marginRight: '6px' }} />
            {t.utilities_filter_streaming}
          </button>
          <button
            className={`subnav-pill ${filter === 'emulators' ? 'active' : ''}`}
            onClick={() => setFilter('emulators')}
          >
            <Gamepad size={14} style={{ marginRight: '6px' }} />
            {t.utilities_filter_emulators}
          </button>
          <button
            className={`subnav-pill ${filter === 'tools' ? 'active' : ''}`}
            onClick={() => setFilter('tools')}
          >
            <Cpu size={14} style={{ marginRight: '6px' }} />
            {t.utilities_filter_tools}
          </button>
        </div>

        <div style={{ position: 'relative', minWidth: '280px' }}>
          <Search
            size={16}
            style={{
              position: 'absolute',
              left: '12px',
              top: '50%',
              transform: 'translateY(-50%)',
              color: 'var(--deck-text-muted)',
            }}
          />
          <input
            type="text"
            className="deck-input focusable"
            placeholder={t.utilities_search_placeholder}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            style={{ paddingLeft: '36px', width: '100%' }}
          />
        </div>
      </div>

      {/* Grid of Cards */}
      <div
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fill, minmax(360px, 1fr))',
          gap: '16px',
        }}
      >
        {filteredItems.map((item) => (
          <div
            key={item.id}
            className="deck-card"
            style={{
              display: 'flex',
              flexDirection: 'column',
              justifyContent: 'space-between',
              padding: '20px',
              backgroundColor: 'var(--deck-surface, #161b24)',
              border: '1px solid var(--deck-border, rgba(255, 255, 255, 0.08))',
              borderRadius: '8px',
            }}
          >
            <div>
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'flex-start',
                  marginBottom: '10px',
                }}
              >
                <div>
                  <h3 style={{ margin: '0 0 4px 0', fontSize: '18px', color: '#fff' }}>
                    {item.name}
                  </h3>
                  <span
                    style={{
                      fontSize: '11px',
                      color: 'var(--deck-focus, #38bdf8)',
                      fontWeight: 600,
                      textTransform: 'uppercase',
                      letterSpacing: '0.5px',
                    }}
                  >
                    {item.categoryLabel[lang === 'pt' ? 'pt' : 'en']}
                  </span>
                </div>
                <span className="badge-tag">{item.tag}</span>
              </div>

              <p
                style={{
                  fontSize: '13.5px',
                  lineHeight: '1.5',
                  color: 'var(--deck-text-main, #d1d5db)',
                  margin: '0 0 12px 0',
                }}
              >
                {item.description[lang === 'pt' ? 'pt' : 'en']}
              </p>

              <div
                style={{
                  backgroundColor: 'rgba(56, 189, 248, 0.06)',
                  borderLeft: '3px solid var(--deck-focus, #38bdf8)',
                  padding: '10px 12px',
                  borderRadius: '4px',
                  marginBottom: '16px',
                }}
              >
                <span
                  style={{
                    fontSize: '11px',
                    fontWeight: 700,
                    color: 'var(--deck-focus, #38bdf8)',
                    display: 'block',
                    marginBottom: '3px',
                    textTransform: 'uppercase',
                  }}
                >
                  💡 {t.utilities_how_to_use}
                </span>
                <span style={{ fontSize: '12px', color: '#9ca3af', lineHeight: '1.4' }}>
                  {item.kameTip[lang === 'pt' ? 'pt' : 'en']}
                </span>
              </div>
            </div>

            <button
              className="deck-btn deck-btn-primary focusable"
              onClick={() => handleOpenUrl(item.url)}
              style={{
                width: '100%',
                justifyContent: 'center',
                padding: '9px 14px',
                gap: '8px',
              }}
            >
              <span>{item.isSteamStore ? t.utilities_btn_steam : t.utilities_btn_visit}</span>
              <ExternalLink size={14} />
            </button>
          </div>
        ))}
      </div>
    </section>
  );
};
