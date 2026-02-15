// Komponente: StreamMonitor — Stream-Sidebar Signal-Analysator
import WaveDisplay from './WaveDisplay';
import VUMeter from '../mixer/VUMeter';

/** Kompakter Signal-Monitor für die Stream-Sidebar */
interface StreamMonitorProps {
  /** Audio-Level Daten */
  levels?: number[];
  /** Peak-Pegel in dB */
  peak?: number;
  /** RMS-Pegel in dB */
  rms?: number;
}

const ORANGE = '#ff8c00'; // Stream-Farbe

/**
 * Kompakter Stream-Monitor für Sidebar
 * - 2 kleine animierte Wellen (Orange)
 * - Stereo VU-Meter
 */
function StreamMonitor({
  levels,
  peak = -60,
  rms = -60,
}: StreamMonitorProps) {
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-2">
      {/* Header */}
      <div className="mb-2 flex items-center justify-between">
        <h4 className="text-[6px] font-extrabold uppercase text-orange-500 tracking-wider">
          Stream Signal
        </h4>

        {/* Stereo VU-Meter (kompakt) */}
        <div className="flex gap-[2px] h-[30px]">
          <VUMeter peak={peak} rms={rms} color="orange" />
          <VUMeter peak={peak} rms={rms} color="orange" />
        </div>
      </div>

      {/* 2 kompakte Wellen */}
      <div className="space-y-1">
        <WaveDisplay
          color={ORANGE}
          duration={2.5}
          waveType={0}
        />

        <WaveDisplay
          color={ORANGE}
          duration={3.2}
          waveType={2}
        />
      </div>

      {/* Stream-Pegel Anzeige */}
      <div className="mt-2 pt-2 border-t border-gray-800 flex items-center justify-between">
        <span className="text-[5px] font-medium uppercase text-gray-600">
          Pegel
        </span>
        <span
          className="text-[6px] font-bold tabular-nums"
          style={{
            color: peak > -10 ? '#ff1744' : peak > -20 ? '#e6a117' : ORANGE,
          }}
        >
          {peak.toFixed(1)} dB
        </span>
      </div>
    </div>
  );
}

export default StreamMonitor;
