// Component: FxModule — Einzelnes FX-Modul (HPF, Gate, etc.)
// Spec: 4×2 Grid-Items, kompakt, 6px ID, 5px Name, Toggle 22×11
import FxSlider from './FxSlider';
import { useFxStore } from '../../stores/fxStore';
import { FX_MODULE_META } from '../../types/fx';
import type { FxModuleInfo } from '../../types/fx';

interface FxModuleProps {
  module: FxModuleInfo;
}

/** FX-Modul mit ID, Name, ON/OFF Toggle, Parameter-Slider */
export default function FxModule({ module }: FxModuleProps) {
  const { setParam, setBypass } = useFxStore();

  const meta = FX_MODULE_META[module.module_type];
  const accentColor = meta.color === 'cyan' ? '#00e5ff' : '#ff8c00';
  const hasAutoButton = ['Gate', 'AIDenoise', 'DeEsser', 'Compressor'].includes(module.module_type);
  const paramValues = Object.fromEntries(module.params);

  return (
    <div
      className="flex flex-col gap-[3px] p-[6px] relative transition-all"
      style={{
        background: module.enabled ? 'rgba(0,229,255,0.015)' : 'rgba(255,255,255,0.01)',
        border: `1px solid ${module.enabled ? (meta.color === 'cyan' ? 'rgba(0,229,255,0.15)' : 'rgba(255,140,0,0.15)') : 'rgba(255,255,255,0.05)'}`,
        borderRadius: '4px',
      }}
    >
      {/* Header: ID + AUTO + Toggle */}
      <div className="flex items-center justify-between">
        <span
          style={{
            fontSize: '6px',
            fontWeight: 700,
            letterSpacing: '0.5px',
            color: module.enabled ? accentColor : 'rgba(255,255,255,0.15)',
          }}
        >
          {meta.shortId || module.module_type}
        </span>
        <div className="flex items-center gap-[2px]">
          {hasAutoButton && (
            <button
              style={{
                padding: '1.5px 4px',
                fontSize: '4.5px',
                fontWeight: 700,
                letterSpacing: '0.5px',
                textTransform: 'uppercase',
                borderRadius: '2px',
                border: '1px solid rgba(0,229,255,0.15)',
                background: 'rgba(0,229,255,0.04)',
                color: '#00e5ff',
                cursor: 'pointer',
              }}
              aria-label="Auto"
            >
              AUTO
            </button>
          )}
          {/* Toggle: 22×11 */}
          <div
            className="cursor-pointer transition-all"
            style={{
              width: '22px',
              height: '11px',
              borderRadius: '6px',
              border: `1px solid ${module.enabled ? '#00e5ff' : 'rgba(255,255,255,0.05)'}`,
              background: module.enabled ? '#00e5ff' : 'rgba(255,255,255,0.03)',
              position: 'relative',
            }}
            onClick={() => setBypass(module.module_type, !module.enabled)}
            role="switch"
            aria-checked={module.enabled}
          >
            <div
              style={{
                width: '7px',
                height: '7px',
                borderRadius: '50%',
                background: '#fff',
                position: 'absolute',
                top: '1px',
                left: module.enabled ? '12px' : '1px',
                transition: 'left 0.15s',
              }}
            />
          </div>
        </div>
      </div>

      {/* Sub-Name: 5px */}
      <div style={{ fontSize: '5px', color: 'rgba(255,255,255,0.06)' }}>
        {meta.name}
      </div>

      {/* Parameter-Slider */}
      {meta.params.map((paramMeta) => {
        const currentValue = paramValues[paramMeta.name] ?? paramMeta.default;

        return (
          <div key={paramMeta.name} className="flex flex-col gap-[1px]">
            {/* Label + Wert auf einer Zeile */}
            <div className="flex justify-between items-center">
              <span
                style={{
                  fontSize: '4.5px',
                  fontWeight: 700,
                  color: 'rgba(255,255,255,0.18)',
                  letterSpacing: '0.5px',
                  textTransform: 'uppercase',
                }}
              >
                {paramMeta.label}
              </span>
              <span
                style={{
                  fontSize: '5px',
                  fontWeight: 600,
                  color: accentColor,
                }}
              >
                {currentValue.toFixed(1)} {paramMeta.unit}
              </span>
            </div>
            {/* Slider */}
            <FxSlider
              value={currentValue}
              min={paramMeta.min}
              max={paramMeta.max}
              onChange={(val) => setParam(module.module_type, paramMeta.name, val)}
              color={meta.color}
              disabled={!module.enabled}
            />
          </div>
        );
      })}

      {meta.params.length === 0 && (
        <div style={{ fontSize: '5px', color: 'rgba(255,255,255,0.1)', textAlign: 'center', padding: '4px 0' }}>
          —
        </div>
      )}
    </div>
  );
}
