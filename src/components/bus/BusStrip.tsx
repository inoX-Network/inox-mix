// Component: BusStrip — Horizontaler Output-Bus Strip
// Spec: min-width 120px, flex:1, ID 9px/800, Sub 5px, Slider h:4px, dB 7px, buttons 5px
import { useBusStore } from '../../stores/busStore';
import type { OutputBus } from '../../types/bus';

interface BusStripProps {
  bus: OutputBus;
}

/**
 * Kompakter horizontaler Bus-Strip (unterhalb Mixer-Strips)
 * Layout: ID + Sub-Label | Slider | dB | MUTE | REC — alles in einer Zeile
 */
export default function BusStrip({ bus }: BusStripProps) {
  const { setMute } = useBusStore();

  const isABus = bus.id.startsWith('A');
  const color = isABus ? '#00e5ff' : '#ff8c00';

  return (
    <div
      className="relative flex items-center gap-[6px] flex-1"
      style={{
        minWidth: '120px',
        background: 'var(--color-inox-strip)',
        border: '1px solid rgba(255,255,255,0.05)',
        borderRadius: '5px',
        padding: '5px 8px',
      }}
    >
      {/* 2px Top-Border in Bus-Farbe */}
      <div
        className="absolute top-0 left-0 right-0"
        style={{ height: '2px', borderRadius: '5px 5px 0 0', opacity: 0.4, background: color }}
      />

      {/* ID + Sub-Label (zentriert, min-width 24px) */}
      <div style={{ textAlign: 'center', minWidth: '24px' }}>
        <div style={{ fontSize: '9px', fontWeight: 800, letterSpacing: '2px', color }}>
          {bus.id}
        </div>
        <div style={{ fontSize: '5px', color: 'rgba(255,255,255,0.13)' }}>
          {bus.name}
        </div>
      </div>

      {/* Horizontaler Mini-Slider (flex:1) */}
      <div className="flex-1 relative" style={{ height: '4px', background: 'rgba(255,255,255,0.03)', borderRadius: '2px' }}>
        <div
          style={{
            height: '100%',
            width: `${((bus.volume_db + 50) / 60) * 100}%`,
            backgroundColor: color,
            borderRadius: '2px',
            boxShadow: `0 0 4px ${color}20`,
          }}
        />
      </div>

      {/* dB-Anzeige */}
      <span style={{ fontSize: '7px', fontWeight: 700, color }}>
        {bus.volume_db === -Infinity ? '-∞' : bus.volume_db.toFixed(1)}
      </span>

      {/* MUTE Button */}
      <button
        style={{
          padding: '1px 6px',
          fontSize: '5px',
          fontWeight: 700,
          letterSpacing: '0.4px',
          textTransform: 'uppercase',
          borderRadius: '2px',
          border: `1px solid ${bus.muted ? 'rgba(255,23,68,0.1)' : 'rgba(255,255,255,0.05)'}`,
          background: bus.muted ? 'rgba(255,23,68,0.06)' : 'rgba(255,255,255,0.01)',
          color: bus.muted ? '#ff1744' : 'rgba(255,255,255,0.18)',
          cursor: 'pointer',
        }}
        onClick={() => setMute(bus.id, !bus.muted)}
        aria-label="Mute"
        aria-pressed={bus.muted}
      >
        MUTE
      </button>

      {/* REC Button */}
      <button
        style={{
          padding: '1px 6px',
          fontSize: '5px',
          fontWeight: 700,
          letterSpacing: '0.4px',
          textTransform: 'uppercase',
          borderRadius: '2px',
          border: `1px solid ${bus.recording ? 'rgba(255,23,68,0.1)' : 'rgba(255,255,255,0.05)'}`,
          background: bus.recording ? 'rgba(255,23,68,0.06)' : 'rgba(255,255,255,0.01)',
          color: bus.recording ? '#ff1744' : 'rgba(255,255,255,0.18)',
          cursor: 'pointer',
        }}
        onClick={() => {}}
        aria-label="Recording"
        aria-pressed={bus.recording}
      >
        ●REC
      </button>
    </div>
  );
}
