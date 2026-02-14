// Component: BusStrip — Horizontaler Output-Bus Strip
import BusSlider from './BusSlider';
import { useBusStore } from '../../stores/busStore';
import type { OutputBus } from '../../types/bus';

interface BusStripProps {
  /** Output-Bus Daten */
  bus: OutputBus;
}

/**
 * Horizontaler Bus-Strip (unterhalb Mixer-Strips)
 */
export default function BusStrip({ bus }: BusStripProps) {
  const { setVolume, setMute } = useBusStore();

  const isABus = bus.id.startsWith('A');
  const color = isABus ? 'cyan' : 'orange';
  const accentColor = isABus ? 'bg-inox-cyan/40' : 'bg-inox-orange/40';
  const labelColor = isABus ? 'text-inox-cyan' : 'text-inox-orange';

  return (
    <div className="min-w-[120px] bg-inox-strip border border-[rgba(255,255,255,0.05)] rounded-[5px] flex flex-col gap-1 p-2">
      {/* Top Accent */}
      <div className={`h-[2px] ${accentColor} rounded-full -mx-1`} />

      {/* ID Label + Sub-Label */}
      <div className="flex flex-col gap-0.5">
        <span className={`text-[8px] font-bold tracking-wider ${labelColor}`}>
          {bus.id}
        </span>
        <span className="text-[6px] text-inox-muted font-medium tracking-wide">
          {bus.name}
        </span>
      </div>

      {/* Volume Slider */}
      <BusSlider
        value={bus.volume_db}
        onChange={(val) => setVolume(bus.id, val)}
        color={color}
        disabled={bus.muted}
      />

      {/* dB Display */}
      <div className={`text-[7px] text-center font-mono ${labelColor}`}>
        {bus.volume_db.toFixed(1)} dB
      </div>

      {/* Buttons: MUTE + REC */}
      <div className="flex gap-1">
        <button
          className={`flex-1 text-[7px] font-bold py-0.5 rounded-sm ${
            bus.muted ? 'bg-inox-red text-white' : 'bg-inox-subtle text-inox-muted'
          }`}
          onClick={() => setMute(bus.id, !bus.muted)}
          aria-label="Mute"
          aria-pressed={bus.muted}
        >
          M
        </button>
        <button
          className={`flex-1 text-[7px] font-bold py-0.5 rounded-sm ${
            bus.recording ? 'bg-inox-red text-white' : 'bg-inox-subtle text-inox-muted'
          }`}
          onClick={() => {}} // TODO: Recording in späterem Modul
          aria-label="Recording"
          aria-pressed={bus.recording}
        >
          REC
        </button>
      </div>
    </div>
  );
}
