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
 * Kompletter Input-Strip mit allen Controls
 */
export default function Strip({ strip }: StripProps) {
  const { setVolume, setGain, setMute, setSolo, setBusRouting, levels } = useMixerStore();

  const isHardware = strip.strip_type === 'Hardware';
  const color = isHardware ? 'cyan' : 'orange';
  const accentColor = isHardware ? 'bg-inox-cyan/45' : 'bg-inox-orange/45';
  const labelColor = isHardware ? 'text-inox-cyan' : 'text-inox-orange';

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
    <div className="min-w-[56px] bg-inox-strip border border-[rgba(255,255,255,0.05)] rounded-[5px] flex flex-col gap-1 p-2 pt-1">
      {/* Dock Handle (6 Dots, Drag & Drop vorbereitet) */}
      <div className="flex gap-0.5 justify-center mb-1 opacity-30 cursor-grab">
        {Array.from({ length: 6 }).map((_, i) => (
          <div key={i} className="w-[2px] h-[2px] bg-inox-subtle rounded-full" />
        ))}
      </div>

      {/* Top Accent */}
      <div className={`h-[2px] ${accentColor} rounded-full -mx-1`} />

      {/* Icon + Label */}
      <div className="flex flex-col items-center gap-0.5">
        <span className="text-[11px]">{strip.icon}</span>
        <span className={`text-[6px] font-bold tracking-wider ${labelColor}`}>{strip.label}</span>
      </div>

      {/* Gain Knob */}
      <Knob
        value={strip.gain_db}
        onChange={(val) => setGain(strip.id, val)}
        label="GAIN"
        color={color}
      />

      {/* VU-Meter + Fader */}
      <div className="flex gap-1 items-center justify-center" style={{ height: '90px' }}>
        {/* VU Meter Links */}
        <VUMeter peak={stripLevels.peak_l} rms={stripLevels.rms_l} color={color} />
        {/* Fader */}
        <Fader
          value={strip.volume_db}
          onChange={(val) => setVolume(strip.id, val)}
          color={color}
          disabled={strip.muted}
        />
        {/* VU Meter Rechts */}
        <VUMeter peak={stripLevels.peak_r} rms={stripLevels.rms_r} color={color} />
      </div>

      {/* dB Display */}
      <div className={`text-[7px] text-center font-mono ${labelColor}`}>
        {strip.volume_db.toFixed(1)} dB
      </div>

      {/* FX Button */}
      <FXButton active={strip.fx_enabled} onClick={() => {}} />

      {/* Bus Routing */}
      <div className="grid grid-cols-2 gap-0.5">
        {buses.map((busId) => (
          <BusButton
            key={busId}
            busId={busId}
            active={strip.bus_routing.includes(busId)}
            onClick={() => setBusRouting(strip.id, busId, !strip.bus_routing.includes(busId))}
          />
        ))}
      </div>

      {/* Mute / Solo */}
      <div className="flex gap-0.5">
        <button
          className={`flex-1 text-[7px] font-bold py-0.5 rounded-sm ${
            strip.muted ? 'bg-inox-red text-white' : 'bg-inox-subtle text-inox-muted'
          }`}
          onClick={() => setMute(strip.id, !strip.muted)}
          aria-label="Mute"
          aria-pressed={strip.muted}
        >
          M
        </button>
        <button
          className={`flex-1 text-[7px] font-bold py-0.5 rounded-sm ${
            strip.solo ? 'bg-inox-amber text-inox-bg' : 'bg-inox-subtle text-inox-muted'
          }`}
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
