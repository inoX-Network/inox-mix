// Component: Mixer — Mixer-Hauptkomponente mit allen Input-Strips
import { useEffect, useState } from 'react';
import Strip from './Strip';
import SignalMonitor from './SignalMonitor';
import BusSection from '../bus/BusSection';
import FxPanel from '../fx/FxPanel';
import MasterSection from '../master/MasterSection';
import { useMixerStore } from '../../stores/mixerStore';

/** Section-Label Komponente (HARDWARE / MASTER / VIRTUAL) */
function SectionLabel({ label, color }: { label: string; color: 'cyan' | 'orange' }) {
  const dotColor = color === 'cyan' ? '#00e5ff' : '#ff8c00';
  return (
    <div className="flex items-center gap-[3px] mb-[3px]">
      <i className="block" style={{ width: '2px', height: '8px', background: dotColor, borderRadius: '1px' }} />
      <span
        style={{
          fontSize: '6px',
          fontWeight: 700,
          letterSpacing: '2.5px',
          color: 'rgba(255,255,255,0.16)',
          textTransform: 'uppercase',
        }}
      >
        {label}
      </span>
    </div>
  );
}

/** Mixer-Hauptkomponente — 3-Spalten-Layout: Hardware | Signal Monitor + Master | Virtual */
export default function Mixer() {
  const { strips, loading, error, loadStrips, addVirtualStrip } = useMixerStore();
  const [showFxPanel, setShowFxPanel] = useState(true);

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
  const hardwareStrips = strips.filter((s) => s.strip_type === 'Hardware');
  const virtualStrips = strips.filter((s) => s.strip_type === 'Virtual');

  return (
    <div className="flex flex-col h-full">
      {/* Top Section: 3-Spalten-Layout */}
      <div className="flex gap-[5px] p-[6px] overflow-x-auto flex-1 items-start">
        {/* LINKS: Hardware-Strips */}
        <div className="shrink-0">
          <SectionLabel label="HARDWARE" color="cyan" />
          <div className="flex gap-[3px]">
            {hardwareStrips.map((strip) => (
              <Strip key={strip.id} strip={strip} />
            ))}
          </div>
        </div>

        {/* Vertikale Trennlinie */}
        <div className="w-px self-stretch shrink-0 bg-gradient-to-b from-transparent via-white/[0.04] to-transparent" />

        {/* MITTE: Signal Monitor + Master nebeneinander */}
        <div className="flex-1 flex gap-[5px] min-w-0 items-start">
          <SignalMonitor />
          <div className="shrink-0">
            <SectionLabel label="MASTER" color="cyan" />
            <MasterSection />
          </div>
        </div>

        {/* Vertikale Trennlinie */}
        <div className="w-px self-stretch shrink-0 bg-gradient-to-b from-transparent via-white/[0.04] to-transparent" />

        {/* RECHTS: Virtual-Strips + Plus-Button */}
        <div className="shrink-0">
          <SectionLabel label="VIRTUAL" color="orange" />
          <div className="flex gap-[3px]">
            {virtualStrips.map((strip) => (
              <Strip key={strip.id} strip={strip} />
            ))}
            {canAddVirtual && (
              <button
                className="flex items-center justify-center transition-all"
                style={{
                  minWidth: '28px',
                  minHeight: '220px',
                  border: '1px dashed rgba(255,255,255,0.06)',
                  background: 'transparent',
                  borderRadius: '5px',
                  color: 'rgba(255,255,255,0.08)',
                  fontSize: '13px',
                }}
                onClick={addVirtualStrip}
                aria-label="Virtual-Strip hinzufügen"
                title="Virtual-Strip hinzufügen (max. 10)"
                onMouseEnter={(e) => {
                  e.currentTarget.style.borderColor = '#00e5ff';
                  e.currentTarget.style.color = '#00e5ff';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.borderColor = 'rgba(255,255,255,0.06)';
                  e.currentTarget.style.color = 'rgba(255,255,255,0.08)';
                }}
              >
                +
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Horizontale Trennlinie */}
      <div className="h-px mx-[6px] bg-gradient-to-r from-transparent via-white/[0.04] to-transparent" style={{ margin: '4px 6px' }} />

      {/* Output Busse */}
      <BusSection />

      {/* FX-Panel */}
      {showFxPanel && (
        <FxPanel onClose={() => setShowFxPanel(false)} />
      )}
    </div>
  );
}
