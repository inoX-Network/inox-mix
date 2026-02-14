// Component: BusSection — Container für alle Output-Busse
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
      <div className="flex items-center justify-center h-16 text-inox-muted text-[7px]">
        Lade Busse...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-16 text-inox-red text-[7px]">
        Fehler: {error}
      </div>
    );
  }

  return (
    <div className="flex gap-1 p-4 pt-2 border-t border-inox-subtle/20">
      {/* Label */}
      <div className="flex items-center">
        <span className="text-[6px] font-bold text-inox-muted tracking-wider uppercase rotate-180" style={{ writingMode: 'vertical-lr' }}>
          Output Busse
        </span>
      </div>

      {/* Bus Strips */}
      {buses.map((bus) => (
        <BusStrip key={bus.id} bus={bus} />
      ))}
    </div>
  );
}
