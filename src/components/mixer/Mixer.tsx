// Component: Mixer — Mixer-Hauptkomponente mit allen Input-Strips
import { useEffect } from 'react';
import Strip from './Strip';
import BusSection from '../bus/BusSection';
import { useMixerStore } from '../../stores/mixerStore';

/**
 * Mixer-Hauptkomponente
 * Lädt Strips vom Backend und rendert sie horizontal
 */
export default function Mixer() {
  const { strips, loading, error, loadStrips, addVirtualStrip } = useMixerStore();

  useEffect(() => {
    loadStrips();
  }, [loadStrips]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full text-inox-muted">
        Lade Mixer...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-full text-inox-red">
        Fehler: {error}
      </div>
    );
  }

  const canAddVirtual = strips.length < 10;

  return (
    <div className="flex flex-col h-full">
      {/* Input Strips Section */}
      <div className="flex gap-2 p-4 overflow-x-auto flex-1">
        {/* Input Strips */}
        {strips.map((strip) => (
          <Strip key={strip.id} strip={strip} />
        ))}

        {/* Plus-Button für Virtual-Strips */}
        {canAddVirtual && (
          <button
            className="min-w-[56px] h-auto bg-inox-panel border-2 border-dashed border-inox-orange/30 rounded-[5px] flex items-center justify-center text-inox-orange hover:border-inox-orange/60 hover:bg-inox-strip transition-all"
            onClick={addVirtualStrip}
            aria-label="Virtual-Strip hinzufügen"
          >
            <span className="text-2xl">+</span>
          </button>
        )}
      </div>

      {/* Output Busse Section */}
      <BusSection />
    </div>
  );
}
