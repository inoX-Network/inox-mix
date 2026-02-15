// Component: Strip — Input Channel Strip (Hardware oder Virtual)
import Fader from './Fader';
import VUMeter from './VUMeter';
import Knob from './Knob';
import BusButton from './BusButton';
import FXButton from './FXButton';
import { useMixerStore } from '../../stores/mixerStore';
import type { InputStrip } from '../../types/mixer';

interface StripProps {
  /** Input-Strip Daten */
  strip: InputStrip;
}

/**
 * Kompletter Input-Strip — Spec: min-width 56px, bg #111318, border-radius 5px
 */
export default function Strip({ strip }: StripProps) {
  const { setVolume, setGain, setMute, setSolo, setBusRouting, levels } = useMixerStore();

  const isHardware = strip.strip_type === 'Hardware';
  const color = isHardware ? 'cyan' : 'orange';
  const accentColor = isHardware ? '#00e5ff' : '#ff8c00';

  const stripLevels = levels[strip.id] || {
    strip_id: strip.id,
    peak_l: -60,
    peak_r: -60,
    rms_l: -60,
    rms_r: -60,
    clipping: false,
  };

  const buses = ['A1', 'A2', 'B1', 'B2'];

  return (
    <div
      className="relative flex flex-col items-center gap-[3px]"
      style={{
        minWidth: '56px',
        padding: '5px 4px',
        background: '#111318',
        borderRadius: '5px',
        border: '1px solid rgba(255,255,255,0.05)',
      }}
    >
      {/* Dock Handle: 6 Dots (2×3), top-right */}
      <div
        className="absolute top-[2px] right-[2px] flex flex-wrap gap-[1px] opacity-0 hover:opacity-30 transition-opacity cursor-grab"
        style={{ width: '8px', height: '8px', alignContent: 'center', justifyContent: 'center' }}
      >
        {Array.from({ length: 6 }).map((_, i) => (
          <i key={i} className="block" style={{ width: '2px', height: '2px', background: 'rgba(255,255,255,0.4)', borderRadius: '50%' }} />
        ))}
      </div>

      {/* Top Accent: 2px Höhe, Kanalfarbe, opacity 45% */}
      <div
        className="absolute top-0 left-0 right-0"
        style={{
          height: '2px',
          borderRadius: '5px 5px 0 0',
          opacity: 0.45,
          background: accentColor,
        }}
      />

      {/* Icon: 11px */}
      <div style={{ fontSize: '11px' }}>{strip.icon}</div>

      {/* Label: 6px, 700 weight, Kanalfarbe, letter-spacing 1px */}
      <div
        style={{
          fontSize: '6px',
          fontWeight: 700,
          color: accentColor,
          letterSpacing: '1px',
        }}
      >
        {strip.label}
      </div>

      {/* Gain Knob: 20px */}
      <Knob
        value={strip.gain_db}
        onChange={(val) => setGain(strip.id, val)}
        label="GAIN"
        color={color}
        size={20}
      />

      {/* VU-Meter + Fader: 75px Höhe */}
      <div className="flex gap-[2px] items-center" style={{ height: '75px' }}>
        <VUMeter peak={stripLevels.peak_l} rms={stripLevels.rms_l} color={color} height={75} />
        <Fader
          value={strip.volume_db}
          onChange={(val) => setVolume(strip.id, val)}
          color={color}
          disabled={strip.muted}
          height={90}
        />
        <VUMeter peak={stripLevels.peak_r} rms={stripLevels.rms_r} color={color} height={75} />
      </div>

      {/* dB-Anzeige: 7px, Kanalfarbe, 600 weight */}
      <div
        style={{
          fontSize: '7px',
          fontWeight: 600,
          color: accentColor,
        }}
      >
        {strip.volume_db.toFixed(1)} <span style={{ fontSize: '5px', opacity: 0.4 }}>dB</span>
      </div>

      {/* FX Button */}
      <FXButton active={strip.fx_enabled} onClick={() => {}} />

      {/* Bus Routing: 4 inline Chips */}
      <div className="flex gap-[1px]">
        {buses.map((busId) => (
          <BusButton
            key={busId}
            busId={busId}
            active={strip.bus_routing.includes(busId)}
            onClick={() => setBusRouting(strip.id, busId, !strip.bus_routing.includes(busId))}
          />
        ))}
      </div>

      {/* Mute / Solo: 5.5px font */}
      <div className="flex gap-[2px]">
        <button
          style={{
            padding: '2px 4px',
            fontSize: '5.5px',
            fontWeight: 700,
            letterSpacing: '0.4px',
            textTransform: 'uppercase',
            borderRadius: '2px',
            border: `1px solid ${strip.muted ? 'transparent' : 'rgba(255,255,255,0.05)'}`,
            background: strip.muted ? '#ff1744' : 'rgba(255,255,255,0.01)',
            color: strip.muted ? '#fff' : 'rgba(255,255,255,0.18)',
          }}
          onClick={() => setMute(strip.id, !strip.muted)}
          aria-label="Mute"
          aria-pressed={strip.muted}
        >
          M
        </button>
        <button
          style={{
            padding: '2px 4px',
            fontSize: '5.5px',
            fontWeight: 700,
            letterSpacing: '0.4px',
            textTransform: 'uppercase',
            borderRadius: '2px',
            border: `1px solid ${strip.solo ? 'transparent' : 'rgba(255,255,255,0.05)'}`,
            background: strip.solo ? '#e6a117' : 'rgba(255,255,255,0.01)',
            color: strip.solo ? '#000' : 'rgba(255,255,255,0.18)',
          }}
          onClick={() => setSolo(strip.id, !strip.solo)}
          aria-label="Solo"
          aria-pressed={strip.solo}
        >
          S
        </button>
      </div>
    </div>
  );
}
