// Component: BusSection — Container für alle Output-Busse
// Spec: .sl label style (2×8px dot, 6px/700/2.5px tracking), gap 4px flex-wrap
import { useEffect } from 'react';
import BusStrip from './BusStrip';
import { useBusStore } from '../../stores/busStore';

/**
 * Bus-Section (unterhalb Mixer-Strips)
 * Zeigt alle Output-Busse horizontal
 */
export default function BusSection() {
  const { buses, loading, error, loadBuses } = useBusStore();

  useEffect(() => {
    loadBuses();
  }, [loadBuses]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-16 text-white/[0.18]" style={{ fontSize: '7px' }}>
        Lade Busse...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-16 text-inox-red" style={{ fontSize: '7px' }}>
        Fehler: {error}
      </div>
    );
  }

  return (
    <div style={{ padding: '0 6px 6px 6px' }}>
      {/* Section-Label — Mockup: .sl style */}
      <div className="flex items-center gap-[4px]" style={{ marginBottom: '3px' }}>
        <i className="block" style={{ width: '2px', height: '8px', background: '#ff8c00', borderRadius: '1px' }} />
        <span
          style={{
            fontSize: '6px',
            fontWeight: 700,
            letterSpacing: '2.5px',
            color: 'rgba(255,255,255,0.16)',
            textTransform: 'uppercase',
          }}
        >
          OUTPUT BUSES
        </span>
      </div>

      {/* Bus Strips (horizontal, flex-wrap) */}
      <div className="flex gap-[4px] flex-wrap">
        {buses.map((bus) => (
          <BusStrip key={bus.id} bus={bus} />
        ))}
      </div>
    </div>
  );
}
