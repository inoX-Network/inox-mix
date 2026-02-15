// Component: VoiceFXTiles — Voice FX als Kachel-Grid (nicht Dropdown)
// Spec: .vfx-tile min-width:36px, icon 13px, name 5px, knob 18px
import { useState } from 'react';
import Knob from '../mixer/Knob';

const voiceFxPresets = [
  { id: 'robot', name: 'Robot', icon: '🤖', color: '#00e5ff' },
  { id: 'vader', name: 'Vader', icon: '👾', color: '#00e5ff' },
  { id: 'chipmunk', name: 'Chipmunk', icon: '🐿️', color: '#00e5ff' },
  { id: 'megaphone', name: 'Mega', icon: '📢', color: '#ff8c00' },
  { id: 'echo', name: 'Echo', icon: '🏔️', color: '#00e5ff' },
  { id: 'radio', name: 'Radio', icon: '📻', color: '#ff8c00' },
];

/** Voice FX Sektion (Kacheln + Dry/Wet Knob) */
export default function VoiceFXTiles() {
  const [activePreset, setActivePreset] = useState<string>('robot');
  const [dryWet, setDryWet] = useState(50);

  return (
    <div
      style={{
        borderRadius: '5px',
        padding: '8px',
        marginBottom: '5px',
        background: 'linear-gradient(135deg, rgba(0,229,255,0.03), rgba(224,64,251,0.03))',
        border: '1px solid rgba(0,229,255,0.06)',
      }}
    >
      {/* Header */}
      <div className="flex items-center justify-between" style={{ marginBottom: '5px' }}>
        <div className="flex items-center gap-[4px]">
          <span style={{ fontSize: '11px' }}>🎭</span>
          <span style={{ fontSize: '7px', fontWeight: 700, letterSpacing: '1.5px', color: 'rgba(255,255,255,0.3)', textTransform: 'uppercase' }}>
            VOICE FX
          </span>
        </div>
        <span
          style={{
            padding: '2px 6px',
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
          ON
        </span>
      </div>

      {/* Kachel-Grid (6 Presets) — .vfx-tile: min-width 36px, gap 3px */}
      <div className="flex gap-[3px] flex-wrap justify-center">
        {voiceFxPresets.map((preset) => (
          <button
            key={preset.id}
            className="inline-flex flex-col items-center gap-[2px] transition-all"
            style={{
              padding: '4px 5px',
              borderRadius: '4px',
              border: `1px solid ${activePreset === preset.id ? `${preset.color}40` : 'rgba(255,255,255,0.05)'}`,
              background: activePreset === preset.id ? 'rgba(255,255,255,0.04)' : 'rgba(255,255,255,0.01)',
              boxShadow: activePreset === preset.id ? `0 0 12px ${preset.color}40` : 'none',
              minWidth: '36px',
              textAlign: 'center',
              cursor: 'pointer',
            }}
            onClick={() => setActivePreset(preset.id)}
          >
            <span style={{ fontSize: '13px' }}>{preset.icon}</span>
            <span
              style={{
                fontSize: '5px',
                fontWeight: 700,
                color: activePreset === preset.id ? preset.color : 'rgba(255,255,255,0.18)',
              }}
            >
              {preset.name}
            </span>
          </button>
        ))}
      </div>

      {/* Dry/Wet Knob + "NUR B1" Label */}
      <div className="flex items-center gap-[4px] justify-center" style={{ marginTop: '4px' }}>
        <Knob value={dryWet} onChange={setDryWet} label="DRY/WET" color="cyan" size={18} min={0} max={100} />
        <span style={{ fontSize: '5px', color: 'rgba(255,255,255,0.06)' }}>NUR B1</span>
      </div>
    </div>
  );
}
