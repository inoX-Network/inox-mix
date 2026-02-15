// Component: FxPanel — FX-Chain Panel (aufklappbar unter Bus-Section)
import { useEffect } from 'react';
import FxModule from './FxModule';
import { useFxStore } from '../../stores/fxStore';

interface FxPanelProps {
  /** Close-Callback */
  onClose: () => void;
}

/**
 * FX-Panel zeigt die FX-Chain horizontal (HPF → Gate → ...)
 * Position: Unterhalb BusSection
 */
export default function FxPanel({ onClose }: FxPanelProps) {
  const { modules, loading, error, loadFxChain } = useFxStore();

  useEffect(() => {
    loadFxChain();
  }, [loadFxChain]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-24 bg-inox-panel border-t border-inox-subtle/20 text-inox-muted text-[7px]">
        Lade FX-Chain...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-24 bg-inox-panel border-t border-inox-subtle/20 text-inox-red text-[7px]">
        Fehler: {error}
      </div>
    );
  }

  return (
    <div className="bg-inox-panel border-t border-inox-subtle/20 p-4">
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span className="text-[8px] font-bold text-inox-cyan tracking-wider uppercase">
            FX-Chain
          </span>
          <span className="text-[6px] text-inox-muted">
            (8 Module: HPF → Denoise → Gate → DeEsser → EQ → Comp → Lim → AutoGain)
          </span>
        </div>
        {/* Close Button */}
        <button
          className="text-[7px] font-bold px-2 py-1 bg-inox-subtle text-inox-muted hover:bg-inox-strip rounded-sm transition-colors"
          onClick={onClose}
          aria-label="FX-Panel schließen"
        >
          ✕ CLOSE
        </button>
      </div>

      {/* FX-Module Horizontal */}
      <div className="flex gap-2 overflow-x-auto">
        {modules.map((module) => (
          <FxModule key={module.module_type} module={module} />
        ))}
      </div>

      {/* Keine Module */}
      {modules.length === 0 && (
        <div className="text-[7px] text-inox-muted text-center py-4">
          Keine FX-Module verfügbar
        </div>
      )}
    </div>
  );
}
