// Component: FxModule — Einzelnes FX-Modul (HPF, Gate, etc.)
import FxSlider from './FxSlider';
import { useFxStore } from '../../stores/fxStore';
import { FX_MODULE_META } from '../../types/fx';
import type { FxModuleInfo } from '../../types/fx';

interface FxModuleProps {
  /** FX-Modul Daten vom Backend */
  module: FxModuleInfo;
}

/**
 * FX-Modul mit ID, Name, ON/OFF Toggle, Parameter-Slider
 */
export default function FxModule({ module }: FxModuleProps) {
  const { setParam, setBypass } = useFxStore();

  // Modul-Metadaten (Name, Farbe, Parameter-Definitionen)
  const meta = FX_MODULE_META[module.module_type];
  const colorClass = meta.color === 'cyan' ? 'text-inox-cyan' : 'text-inox-orange';
  const accentClass = meta.color === 'cyan' ? 'bg-inox-cyan/40' : 'bg-inox-orange/40';
  const borderClass = meta.color === 'cyan' ? 'border-inox-cyan/20' : 'border-inox-orange/20';

  // Parameter-Werte aus Backend-Daten extrahieren
  const paramValues = Object.fromEntries(module.params);

  return (
    <div className={`min-w-[140px] bg-inox-strip border ${borderClass} rounded-[5px] flex flex-col gap-1 p-2`}>
      {/* Top Accent */}
      <div className={`h-[2px] ${accentClass} rounded-full -mx-1`} />

      {/* ID Label (z.B. "HPF", "GATE") */}
      <div className="flex items-center justify-between">
        <span className={`text-[8px] font-bold tracking-wider ${colorClass}`}>
          {meta.name}
        </span>
        {/* ON/OFF Toggle */}
        <button
          className={`text-[6px] font-bold px-1 py-0.5 rounded-sm ${
            module.enabled
              ? `${accentClass} ${colorClass}`
              : 'bg-inox-subtle text-inox-muted'
          }`}
          onClick={() => setBypass(module.module_type, !module.enabled)}
          aria-label="Toggle"
          aria-pressed={module.enabled}
        >
          {module.enabled ? 'ON' : 'OFF'}
        </button>
      </div>

      {/* Parameter-Slider */}
      <div className="flex flex-col gap-1">
        {meta.params.map((paramMeta) => {
          const currentValue = paramValues[paramMeta.name] ?? paramMeta.default;

          return (
            <div key={paramMeta.name} className="flex flex-col gap-0.5">
              {/* Label */}
              <span className={`text-[7px] font-medium uppercase tracking-wide ${colorClass}`} style={{ opacity: 0.6 }}>
                {paramMeta.label}
              </span>
              {/* Slider */}
              <FxSlider
                value={currentValue}
                min={paramMeta.min}
                max={paramMeta.max}
                onChange={(val) => setParam(module.module_type, paramMeta.name, val)}
                color={meta.color}
                disabled={!module.enabled}
              />
              {/* Wert-Anzeige */}
              <span className={`text-[7px] text-center font-mono ${colorClass}`}>
                {currentValue.toFixed(1)} {paramMeta.unit}
              </span>
            </div>
          );
        })}
      </div>

      {/* Falls keine Parameter: Placeholder */}
      {meta.params.length === 0 && (
        <div className="text-[6px] text-inox-muted text-center py-2">
          Noch nicht implementiert
        </div>
      )}
    </div>
  );
}
