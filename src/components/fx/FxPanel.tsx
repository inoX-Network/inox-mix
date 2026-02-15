// Component: FxPanel — FX-Chain Panel (aufklappbar unter Bus-Section)
// Spec: Grid 4×2, kompakte Chips/Arrows, Quick Calibrate
import { useEffect } from 'react';
import FxModule from './FxModule';
import { useFxStore } from '../../stores/fxStore';
import { FX_MODULE_META } from '../../types/fx';

interface FxPanelProps {
  onClose: () => void;
}

/** FX-Panel zeigt die FX-Chain — Position: Unterhalb BusSection */
export default function FxPanel({ onClose }: FxPanelProps) {
  const { modules, loading, error, loadFxChain } = useFxStore();

  useEffect(() => {
    loadFxChain();
  }, [loadFxChain]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-24 bg-inox-panel border-t border-white/[0.05] text-white/[0.18]" style={{ fontSize: '7px' }}>
        Lade FX-Chain...
      </div>
    );
  }

  if (error) {
    return (
      <div className="flex items-center justify-center h-24 bg-inox-panel border-t border-white/[0.05] text-inox-red" style={{ fontSize: '7px' }}>
        Fehler: {error}
      </div>
    );
  }

  return (
    <div className="relative bg-inox-panel border border-[rgba(0,229,255,0.08)] rounded-md p-2 mx-[6px] mb-[6px] overflow-hidden">
      {/* Top Gradient-Divider (Cyan→Orange) */}
      <div
        className="absolute top-0 left-0 right-0 h-[2px] opacity-30"
        style={{ background: 'linear-gradient(90deg, #00e5ff, #ff8c00)' }}
      />

      {/* Header */}
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-[6px]">
          <span style={{ fontSize: '13px' }}>🎙️</span>
          <div>
            <div style={{ fontSize: '9px', fontWeight: 800, color: '#00e5ff', letterSpacing: '1.5px' }}>
              FX CHAIN — USB MIC
            </div>
            <div style={{ fontSize: '5px', color: 'rgba(255,255,255,0.08)' }}>
              8-Stage Signal Processing
            </div>
          </div>
        </div>
        {/* Quick Calibrate */}
        <button
          style={{
            padding: '4px 10px',
            fontSize: '6px',
            fontWeight: 700,
            letterSpacing: '1px',
            textTransform: 'uppercase',
            borderRadius: '3px',
            border: '1px solid rgba(0,229,255,0.2)',
            background: 'linear-gradient(135deg, rgba(0,229,255,0.08), rgba(224,64,251,0.08))',
            color: '#00e5ff',
            cursor: 'pointer',
            display: 'inline-flex',
            alignItems: 'center',
            gap: '4px',
          }}
          aria-label="Quick Calibrate"
        >
          🎙️ QUICK CALIBRATE
        </button>
        {/* Close */}
        <button
          style={{
            background: 'none',
            border: '1px solid rgba(255,255,255,0.05)',
            color: 'rgba(255,255,255,0.2)',
            padding: '3px 6px',
            borderRadius: '3px',
            fontSize: '7px',
            cursor: 'pointer',
            marginLeft: '6px',
          }}
          onClick={onClose}
          aria-label="FX-Panel schließen"
        >
          ✖ CLOSE
        </button>
      </div>

      {/* Chain-Flow: HPF → AI-DN → GATE → DE-S → EQ → COMP → LIM → A-G */}
      <div className="flex items-center gap-[2px] mb-[6px] overflow-x-auto pb-[2px]">
        {(['Hpf', 'Denoise', 'Gate', 'DeEsser', 'Eq', 'Compressor', 'Limiter', 'AutoGain'] as const).map((modType, i, arr) => {
          const meta = FX_MODULE_META[modType];
          const id = meta.shortId;
          const isOn = modules.find((m) => m.module_type === modType)?.enabled;
          const chipColor = meta.color === 'cyan' ? '#00e5ff' : '#ff8c00';
          return (
            <div key={id} className="flex items-center gap-[2px]">
              <span
                style={{
                  fontSize: '5.5px',
                  fontWeight: 700,
                  letterSpacing: '0.5px',
                  padding: '2px 5px',
                  borderRadius: '2px',
                  border: `1px solid ${isOn ? chipColor + '40' : 'rgba(255,255,255,0.05)'}`,
                  color: isOn ? chipColor : 'rgba(255,255,255,0.12)',
                  background: isOn ? chipColor + '08' : 'transparent',
                  whiteSpace: 'nowrap',
                }}
              >
                {id}
              </span>
              {i < arr.length - 1 && (
                <span style={{ color: 'rgba(255,255,255,0.06)', fontSize: '7px' }}>→</span>
              )}
            </div>
          );
        })}
      </div>

      {/* Module Grid: 4×2 */}
      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '4px' }}>
        {modules.map((module) => (
          <FxModule key={module.module_type} module={module} />
        ))}
      </div>

      {modules.length === 0 && (
        <div style={{ fontSize: '7px', color: 'rgba(255,255,255,0.18)', textAlign: 'center', padding: '12px 0' }}>
          Keine FX-Module verfügbar
        </div>
      )}
    </div>
  );
}
