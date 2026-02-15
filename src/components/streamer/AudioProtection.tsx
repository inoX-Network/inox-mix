// Component: AudioProtection — Kombinierte Ducking + Bleeper Box
// Spec: ModeTile icon 10px, name 4px, min-width 30px, gap 2px
import { useState } from 'react';

/** Audio Protection Box (Ducking + Bleeper kombiniert) */
export default function AudioProtection() {
  // Ducking State
  const [duckAmount, setDuckAmount] = useState(70);
  const [duckAttack, setDuckAttack] = useState(20);
  const [duckRelease, setDuckRelease] = useState(50);
  const [duckThresh, setDuckThresh] = useState(40);

  // Bleeper State
  const [bleeperEngine, setBleeperEngine] = useState<'VOSK' | 'WHISPER'>('VOSK');
  const [bleeperLangDE, setBleeperLangDE] = useState(true);
  const [bleeperLangEN, setBleeperLangEN] = useState(true);
  const [bleeperMode, setBleeperMode] = useState<number>(0);
  const [bleeperTone, setBleeperTone] = useState(50);
  const [bleeperVol, setBleeperVol] = useState(80);

  const bleeperModes = [
    { name: 'Beep', icon: '🔊' },
    { name: 'Mute', icon: '🔇' },
    { name: 'Noise', icon: '🌫️' },
    { name: 'Reverse', icon: '🔃' },
    { name: 'Custom', icon: '🎵' },
  ];

  return (
    <div
      style={{
        borderRadius: '5px',
        padding: '8px',
        marginBottom: '5px',
        background: 'linear-gradient(135deg, rgba(255,109,0,0.03), rgba(255,23,68,0.03))',
        border: '1px solid rgba(255,109,0,0.06)',
      }}
    >
      {/* Section Header */}
      <div className="flex items-center justify-between" style={{ marginBottom: '6px' }}>
        <span style={{ fontSize: '7px', fontWeight: 700, letterSpacing: '1.5px', color: 'rgba(255,255,255,0.25)', textTransform: 'uppercase' }}>
          AUDIO PROTECTION
        </span>
        <div className="flex gap-[3px]">
          <span
            style={{
              padding: '2px 5px',
              fontSize: '5px',
              fontWeight: 700,
              letterSpacing: '0.4px',
              textTransform: 'uppercase',
              borderRadius: '2px',
              background: '#00e5ff',
              border: '1px solid #00e5ff',
              color: '#000',
            }}
          >
            DUCK
          </span>
          <span
            style={{
              padding: '2px 5px',
              fontSize: '5px',
              fontWeight: 700,
              letterSpacing: '0.4px',
              textTransform: 'uppercase',
              borderRadius: '2px',
              background: '#ff1744',
              border: '1px solid #ff1744',
              color: '#fff',
            }}
          >
            BLEEP
          </span>
        </div>
      </div>

      {/* ── DUCKING ── */}
      <div style={{ marginBottom: '6px' }}>
        <div style={{ fontSize: '5px', fontWeight: 700, color: '#00e5ff', letterSpacing: '1px', marginBottom: '3px', display: 'flex', alignItems: 'center', gap: '3px' }}>
          <span style={{ width: '8px', height: '1px', background: 'rgba(0,229,255,0.3)', display: 'inline-block' }} />
          <span>DUCKING</span>
          <span style={{ flex: 1, height: '1px', background: 'rgba(0,229,255,0.15)', display: 'inline-block' }} />
        </div>

        {/* 2×2 Grid Slider */}
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '4px 6px', marginBottom: '4px' }}>
          <MiniSlider label="AMOUNT" value={duckAmount} onChange={setDuckAmount} color="#00e5ff" />
          <MiniSlider label="ATTACK" value={duckAttack} onChange={setDuckAttack} color="#00e5ff" />
          <MiniSlider label="RELEASE" value={duckRelease} onChange={setDuckRelease} color="#00e5ff" />
          <MiniSlider label="THRESH" value={duckThresh} onChange={setDuckThresh} color="#00e5ff" />
        </div>

        {/* Trigger + Target Chips */}
        <div className="flex items-center gap-[2px] flex-wrap">
          <span style={{ fontSize: '4.5px', color: 'rgba(255,255,255,0.08)' }}>TRG:</span>
          <Chip label="MIC" active color="#00e5ff" />
          <span style={{ fontSize: '4.5px', color: 'rgba(255,255,255,0.08)', marginLeft: '3px' }}>TGT:</span>
          <Chip label="Spotify" active color="#ff8c00" />
          <Chip label="Game" active color="#ff8c00" />
        </div>
      </div>

      {/* Gradient-Divider */}
      <div
        style={{
          height: '1px',
          background: 'linear-gradient(90deg, transparent, rgba(255,255,255,0.04), transparent)',
          margin: '6px 0',
        }}
      />

      {/* ── BLEEPER ── */}
      <div>
        <div style={{ fontSize: '5px', fontWeight: 700, color: '#ff1744', letterSpacing: '1px', marginBottom: '3px', display: 'flex', alignItems: 'center', gap: '3px' }}>
          <span style={{ width: '8px', height: '1px', background: 'rgba(255,23,68,0.3)', display: 'inline-block' }} />
          <span>BLEEPER</span>
          <span style={{ flex: 1, height: '1px', background: 'rgba(255,23,68,0.15)', display: 'inline-block' }} />
        </div>

        {/* Engine + Lang Chips */}
        <div className="flex gap-[2px] flex-wrap" style={{ marginBottom: '4px' }}>
          <Chip
            label="VOSK"
            active={bleeperEngine === 'VOSK'}
            onClick={() => setBleeperEngine('VOSK')}
            color="#ff1744"
            activeTextColor="#fff"
          />
          <Chip
            label="WHISPER"
            active={bleeperEngine === 'WHISPER'}
            onClick={() => setBleeperEngine('WHISPER')}
            color="#ff1744"
            activeTextColor="#fff"
          />
          <span style={{ flex: 1 }} />
          <Chip
            label="🇩🇪"
            active={bleeperLangDE}
            onClick={() => setBleeperLangDE(!bleeperLangDE)}
            color="#ff1744"
            activeTextColor="#fff"
          />
          <Chip
            label="🇬🇧"
            active={bleeperLangEN}
            onClick={() => setBleeperLangEN(!bleeperLangEN)}
            color="#ff1744"
            activeTextColor="#fff"
          />
        </div>

        {/* Modus-Label */}
        <div style={{ fontSize: '4.5px', fontWeight: 700, color: 'rgba(255,255,255,0.1)', letterSpacing: '0.6px', marginBottom: '2px', textTransform: 'uppercase' }}>
          MODUS
        </div>

        {/* 5 Modus-Kacheln — .vfx-tile style: min-width 30px, icon 10px, name 4px */}
        <div className="flex gap-[2px] flex-wrap" style={{ marginBottom: '4px' }}>
          {bleeperModes.map((mode, i) => (
            <ModeTile
              key={i}
              icon={mode.icon}
              name={mode.name}
              active={bleeperMode === i}
              onClick={() => setBleeperMode(i)}
            />
          ))}
        </div>

        {/* Kategorie-Chips */}
        <div className="flex gap-[2px] flex-wrap" style={{ marginBottom: '4px' }}>
          {['Schimpf', 'Beleid.', 'Rass.', 'Custom'].map((cat) => (
            <Chip key={cat} label={cat} active color="rgba(255,23,68,0.1)" activeTextColor="#ff1744" />
          ))}
        </div>

        {/* Tone + Vol Slider */}
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '4px 6px' }}>
          <MiniSlider label="TONE" value={bleeperTone} onChange={setBleeperTone} color="#ff8c00" />
          <MiniSlider label="VOL" value={bleeperVol} onChange={setBleeperVol} color="#ff8c00" />
        </div>
      </div>
    </div>
  );
}

/** Mini-Slider für Ducking/Bleeper */
function MiniSlider({
  label,
  value,
  onChange,
  color,
}: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  color: string;
}) {
  return (
    <div className="flex flex-col gap-[1px]">
      <div className="flex justify-between items-center">
        <span style={{ fontSize: '4.5px', fontWeight: 700, color: 'rgba(255,255,255,0.18)', letterSpacing: '0.5px', textTransform: 'uppercase' }}>
          {label}
        </span>
        <span style={{ fontSize: '5px', fontWeight: 600, color }}>{value}%</span>
      </div>
      <input
        type="range"
        min="0"
        max="100"
        value={value}
        onChange={(e) => onChange(parseInt(e.target.value))}
        className="w-full h-[5px] rounded-[3px] appearance-none cursor-pointer"
        style={{
          background: `linear-gradient(to right, ${color} ${value}%, rgba(255,255,255,0.03) ${value}%)`,
        }}
      />
    </div>
  );
}

/** Chip-Button */
function Chip({
  label,
  active,
  onClick,
  color = '#00e5ff',
  activeTextColor,
}: {
  label: string;
  active?: boolean;
  onClick?: () => void;
  color?: string;
  activeTextColor?: string;
}) {
  return (
    <button
      style={{
        padding: '1.5px 4px',
        fontSize: '5px',
        fontWeight: 700,
        letterSpacing: '0.4px',
        textTransform: 'uppercase',
        borderRadius: '2px',
        border: `1px solid ${active ? color : 'rgba(255,255,255,0.05)'}`,
        background: active ? color : 'transparent',
        color: active ? (activeTextColor || '#000') : 'rgba(255,255,255,0.18)',
        cursor: onClick ? 'pointer' : 'default',
        whiteSpace: 'nowrap',
        display: 'inline-block',
      }}
      onClick={onClick}
    >
      {label}
    </button>
  );
}

/** Modus-Kachel — Mockup: .vfx-tile min-width:30px, icon 10px, name 4px */
function ModeTile({
  icon,
  name,
  active,
  onClick,
}: {
  icon: string;
  name: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      className="inline-flex flex-col items-center gap-[2px] transition-all"
      style={{
        minWidth: '30px',
        padding: '3px 4px',
        borderRadius: '4px',
        border: `1px solid ${active ? 'rgba(255,23,68,0.25)' : 'rgba(255,255,255,0.05)'}`,
        background: active ? 'rgba(255,255,255,0.04)' : 'rgba(255,255,255,0.01)',
        boxShadow: active ? '0 0 12px rgba(255,23,68,0.25)' : 'none',
        cursor: 'pointer',
        textAlign: 'center',
      }}
      onClick={onClick}
    >
      <span style={{ fontSize: '10px' }}>{icon}</span>
      <span
        style={{
          fontSize: '4px',
          fontWeight: 700,
          color: active ? '#ff1744' : 'rgba(255,255,255,0.15)',
        }}
      >
        {name}
      </span>
    </button>
  );
}
