// Component: RoutingMatrix — Kreuzmatrix für Audio-Routing (Input/App → Bus)
import { useEffect } from 'react';
import { useRoutingStore } from '../../stores/routingStore';
import { useMixerStore } from '../../stores/mixerStore';
import { useBusStore } from '../../stores/busStore';

/**
 * Routing-Matrix: Visuelle Kreuzmatrix
 * Zeilen: Input-Strips
 * Spalten: Output-Busse (A1, A2, B1, B2)
 */
export default function RoutingMatrix() {
  const { loading, error, loadRoutingMatrix, setRouting, isRouted } = useRoutingStore();
  const { strips } = useMixerStore();
  const { buses } = useBusStore();

  useEffect(() => {
    loadRoutingMatrix();
  }, [loadRoutingMatrix]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full text-inox-muted text-[7px]">
        Lade Routing-Matrix...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-full text-inox-red text-[7px]">
        Fehler: {error}
      </div>
    );
  }

  // Matrix-Daten vorbereiten
  const sources = strips.map((s) => ({ id: s.id, label: s.label, type: 'strip' as const }));

  return (
    <div className="p-6 overflow-auto h-full">
      {/* Header */}
      <div className="mb-4">
        <h1 className="text-[10px] font-bold text-inox-cyan tracking-wider uppercase">
          Routing Matrix
        </h1>
        <p className="text-[6px] text-inox-muted mt-1">
          Klicke auf Kreuzungspunkte um Audio-Routing zu aktivieren/deaktivieren
        </p>
      </div>

      {/* Matrix-Tabelle */}
      <div className="inline-block min-w-full">
        <table className="border-collapse">
          <thead>
            <tr>
              {/* Leere Ecke */}
              <th className="p-2 border border-inox-subtle/20 bg-inox-panel">
                <span className="text-[6px] text-inox-muted uppercase tracking-wide">
                  Source / Bus
                </span>
              </th>
              {/* Bus-Spalten */}
              {buses.map((bus) => (
                <th
                  key={bus.id}
                  className="p-2 border border-inox-subtle/20 bg-inox-panel text-center min-w-[80px]"
                >
                  <div className="flex flex-col gap-0.5">
                    <span
                      className={`text-[8px] font-bold tracking-wider ${
                        bus.id.startsWith('A') ? 'text-inox-cyan' : 'text-inox-orange'
                      }`}
                    >
                      {bus.id}
                    </span>
                    <span className="text-[6px] text-inox-muted">{bus.name}</span>
                  </div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {/* Source-Zeilen */}
            {sources.map((source) => (
              <tr key={source.id}>
                {/* Source-Label */}
                <td className="p-2 border border-inox-subtle/20 bg-inox-panel">
                  <div className="flex items-center gap-2">
                    <span className="text-[7px] font-medium text-inox-text">
                      {source.label}
                    </span>
                    <span className="text-[5px] text-inox-muted uppercase">
                      ({source.id})
                    </span>
                  </div>
                </td>
                {/* Bus-Kreuzungspunkte */}
                {buses.map((bus) => {
                  const routed = isRouted(source.id, bus.id);
                  const busColor = bus.id.startsWith('A') ? 'cyan' : 'orange';
                  const activeBg = busColor === 'cyan' ? 'bg-inox-cyan/20' : 'bg-inox-orange/20';
                  const activeBorder = busColor === 'cyan' ? 'border-inox-cyan/40' : 'border-inox-orange/40';
                  const activeText = busColor === 'cyan' ? 'text-inox-cyan' : 'text-inox-orange';

                  return (
                    <td
                      key={bus.id}
                      className="p-2 border border-inox-subtle/20 bg-inox-bg text-center"
                    >
                      <button
                        className={`w-12 h-12 rounded-sm border-2 flex items-center justify-center transition-all ${
                          routed
                            ? `${activeBg} ${activeBorder} ${activeText}`
                            : 'bg-inox-panel border-inox-subtle/20 text-inox-subtle hover:border-inox-subtle/40'
                        }`}
                        onClick={() => setRouting(source.id, bus.id, !routed)}
                        aria-label={`Routing ${source.label} → ${bus.id}`}
                        aria-pressed={routed}
                      >
                        {routed && (
                          <span className="text-[14px] font-bold">✓</span>
                        )}
                      </button>
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Keine Sources */}
      {sources.length === 0 && (
        <div className="text-[7px] text-inox-muted text-center py-8">
          Keine Input-Strips verfügbar
        </div>
      )}
    </div>
  );
}
