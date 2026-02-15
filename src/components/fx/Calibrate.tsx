// Komponente: Calibrate — Quick Calibrate UI (automatische Mikrofon-Einstellung)
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

/** Kalibrierungs-Ergebnis vom Backend */
interface CalibrationResult {
  recommended_gain_db: number;
  noise_floor_db: number;
  recommended_gate_db: number;
  recommended_hpf_hz: number;
}

/** Kalibrierungs-Status */
type CalibrationState = 'idle' | 'recording' | 'analyzing' | 'complete';

/** Quick Calibrate Dialog — misst Raum und empfiehlt Einstellungen */
interface CalibrateProps {
  /** Kanal-ID der kalibriert wird */
  channelId: string;
  /** Callback wenn Kalibrierung abgeschlossen */
  onComplete?: (result: CalibrationResult) => void;
  /** Callback zum Schließen */
  onCancel?: () => void;
}

function Calibrate({ channelId, onComplete, onCancel }: CalibrateProps) {
  const [state, setState] = useState<CalibrationState>('idle');
  const [progress, setProgress] = useState(0);
  const [result, setResult] = useState<CalibrationResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleStartCalibration = async () => {
    setState('recording');
    setProgress(0);
    setError(null);

    // Fortschrittsbalken simulieren (10 Sekunden)
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          return 100;
        }
        return prev + 10;
      });
    }, 1000);

    try {
      // Nach 10 Sekunden: Analyse
      setTimeout(() => {
        setState('analyzing');
      }, 10000);

      // Kalibrierung durchführen
      const calibrationResult = await invoke<CalibrationResult>('run_calibration', {
        channelId,
      });

      setState('complete');
      setResult(calibrationResult);
    } catch (err) {
      setState('idle');
      setError(String(err));
      clearInterval(interval);
    }
  };

  const handleApply = () => {
    if (result && onComplete) {
      onComplete(result);
    }
  };

  const handleCancel = () => {
    if (onCancel) {
      onCancel();
    }
  };

  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-4 max-w-md">
      {/* Header */}
      <div className="mb-4">
        <h3 className="text-[9px] font-extrabold uppercase tracking-wider text-cyan-500">
          🎙️ Quick Calibrate
        </h3>
        <p className="text-[5px] text-gray-500 mt-1">
          Automatische Mikrofon-Kalibrierung für optimalen Sound
        </p>
      </div>

      {/* Idle State */}
      {state === 'idle' && (
        <div className="space-y-4">
          <div className="p-3 bg-cyan-500/10 border border-cyan-500/30 rounded">
            <p className="text-[6px] text-cyan-400 mb-2 font-medium">
              Anleitung:
            </p>
            <ol className="text-[5px] text-gray-400 space-y-1 list-decimal list-inside">
              <li>Klicke auf "START KALIBRIERUNG"</li>
              <li>Sprich 10 Sekunden normal ins Mikrofon</li>
              <li>Warte auf die Analyse</li>
              <li>Übernimm die empfohlenen Einstellungen</li>
            </ol>
          </div>

          <button
            onClick={handleStartCalibration}
            className="w-full px-4 py-3 bg-cyan-500 hover:bg-cyan-600 text-background font-bold text-[6px] uppercase tracking-wide rounded transition-colors"
          >
            Start Kalibrierung
          </button>
        </div>
      )}

      {/* Recording State */}
      {state === 'recording' && (
        <div className="space-y-4">
          <div className="text-center">
            <div className="text-[8px] font-bold text-cyan-500 mb-2">
              🎤 Sprich jetzt...
            </div>
            <p className="text-[5px] text-gray-400">
              Sprich normal ins Mikrofon. Noch {Math.ceil((100 - progress) / 10)} Sekunden.
            </p>
          </div>

          {/* Fortschrittsbalken */}
          <div className="space-y-1">
            <div className="flex items-center justify-between text-[5px]">
              <span className="text-gray-500">Fortschritt</span>
              <span className="text-cyan-500 font-bold">{progress}%</span>
            </div>
            <div className="w-full h-2 bg-gray-800 rounded-full overflow-hidden">
              <div
                className="h-full bg-cyan-500 transition-all duration-1000"
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>

          {/* Wellenform-Simulation */}
          <div className="h-16 bg-gray-900 rounded flex items-center justify-center">
            <div className="flex items-center gap-1">
              {Array.from({ length: 20 }).map((_, i) => (
                <div
                  key={i}
                  className="w-1 bg-cyan-500 rounded animate-pulse"
                  style={{
                    height: `${20 + Math.random() * 40}px`,
                    animationDelay: `${i * 0.1}s`,
                  }}
                />
              ))}
            </div>
          </div>
        </div>
      )}

      {/* Analyzing State */}
      {state === 'analyzing' && (
        <div className="text-center py-8">
          <div className="w-12 h-12 border-4 border-cyan-500 border-t-transparent rounded-full animate-spin mx-auto mb-3" />
          <p className="text-[6px] text-cyan-500 font-medium">
            Analysiere Audio-Signal...
          </p>
        </div>
      )}

      {/* Complete State */}
      {state === 'complete' && result && (
        <div className="space-y-4">
          <div className="p-3 bg-green-500/10 border border-green-500/30 rounded">
            <p className="text-[6px] text-green-400 font-medium mb-2">
              ✓ Kalibrierung abgeschlossen
            </p>
          </div>

          {/* Ergebnis-Anzeige */}
          <div className="space-y-2">
            <h4 className="text-[6px] font-bold text-gray-300 uppercase">
              Empfohlene Einstellungen:
            </h4>

            <div className="grid grid-cols-2 gap-2">
              <div className="p-2 bg-gray-800 rounded">
                <div className="text-[4.5px] text-gray-500 uppercase">Gain</div>
                <div className="text-[7px] font-bold text-cyan-500">
                  {result.recommended_gain_db >= 0 ? '+' : ''}
                  {result.recommended_gain_db.toFixed(1)} dB
                </div>
              </div>

              <div className="p-2 bg-gray-800 rounded">
                <div className="text-[4.5px] text-gray-500 uppercase">Gate</div>
                <div className="text-[7px] font-bold text-cyan-500">
                  {result.recommended_gate_db.toFixed(1)} dB
                </div>
              </div>

              <div className="p-2 bg-gray-800 rounded">
                <div className="text-[4.5px] text-gray-500 uppercase">HPF</div>
                <div className="text-[7px] font-bold text-cyan-500">
                  {result.recommended_hpf_hz.toFixed(0)} Hz
                </div>
              </div>

              <div className="p-2 bg-gray-800 rounded">
                <div className="text-[4.5px] text-gray-500 uppercase">Noise Floor</div>
                <div className="text-[7px] font-bold text-gray-500">
                  {result.noise_floor_db.toFixed(1)} dB
                </div>
              </div>
            </div>
          </div>

          {/* Buttons */}
          <div className="flex gap-2">
            <button
              onClick={handleCancel}
              className="flex-1 px-4 py-2 bg-gray-800 hover:bg-gray-700 text-gray-400 font-medium text-[5px] uppercase rounded transition-colors"
            >
              Verwerfen
            </button>
            <button
              onClick={handleApply}
              className="flex-1 px-4 py-2 bg-cyan-500 hover:bg-cyan-600 text-background font-bold text-[5px] uppercase rounded transition-colors"
            >
              Übernehmen
            </button>
          </div>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="mt-4 p-3 bg-red-500/10 border border-red-500/30 rounded">
          <p className="text-[5px] text-red-400">{error}</p>
        </div>
      )}
    </div>
  );
}

export default Calibrate;
