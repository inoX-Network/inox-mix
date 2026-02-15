// Komponente: SignalMonitor — Haupt-Signal-Analysator mit 4 animierten Wellen
import WaveDisplay from './WaveDisplay';

/** Signal-Monitor mit 4 animierten SVG-Wellen (2x Cyan, 2x Orange) */
interface SignalMonitorProps {
  /** Audio-Level Daten für die Wellen (optional, für zukünftige Echtzeit-Anzeige) */
  levels?: number[];
  /** Kompakt-Modus (kleinere Höhe) */
  compact?: boolean;
}

const CYAN = '#00e5ff';
const ORANGE = '#ff8c00';

/**
 * Signal-Monitor mit 4 animierten Wellen
 * - Welle 1+2: Cyan (Hardware Inputs)
 * - Welle 3+4: Orange (Virtual Inputs)
 */
function SignalMonitor({ levels: _levels, compact = false }: SignalMonitorProps) {
  return (
    <div
      className={`bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] ${
        compact ? 'p-2' : 'p-3'
      }`}
    >
      {/* Header */}
      {!compact && (
        <div className="mb-3">
          <h3 className="text-[7px] font-extrabold uppercase text-gray-400 tracking-wider">
            Signal Monitor
          </h3>
          <p className="text-[5px] text-gray-600 mt-0.5">
            Echtzeit-Signalanalyse
          </p>
        </div>
      )}

      {/* 4 Wellen */}
      <div className={`space-y-${compact ? '1' : '2'}`}>
        {/* Welle 1: Cyan (Hardware Input 1) */}
        <WaveDisplay
          color={CYAN}
          label="HW 1"
          duration={2.8}
          waveType={0}
        />

        {/* Welle 2: Cyan (Hardware Input 2) */}
        <WaveDisplay
          color={CYAN}
          label="HW 2"
          duration={3.2}
          waveType={1}
        />

        {/* Welle 3: Orange (Virtual Input 1) */}
        <WaveDisplay
          color={ORANGE}
          label="VIRT 1"
          duration={3.5}
          waveType={2}
        />

        {/* Welle 4: Orange (Virtual Input 2) */}
        <WaveDisplay
          color={ORANGE}
          label="VIRT 2"
          duration={2.6}
          waveType={3}
        />
      </div>

      {/* Footer Stats (optional) */}
      {!compact && (
        <div className="mt-3 pt-2 border-t border-gray-800 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-[5px] font-semibold uppercase text-gray-600">
              Status
            </span>
            <div className="flex items-center gap-1">
              <div className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse" />
              <span className="text-[5px] text-green-500 font-medium">
                Live
              </span>
            </div>
          </div>

          <div className="text-[5px] text-gray-600 font-medium tabular-nums">
            48kHz · 256 samples
          </div>
        </div>
      )}
    </div>
  );
}

export default SignalMonitor;
