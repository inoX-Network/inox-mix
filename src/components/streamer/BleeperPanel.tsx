// Komponente: BleeperPanel — Profanity Bleeper Einstellungen
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Slider from '../mixer/Slider';

/** Bleeper-Modi */
type BleeperMode = 'Beep' | 'Mute' | 'Noise' | 'Reverse' | 'Custom';

/** Bleeper-Panel mit Modus-Auswahl, Tone und Volume */
interface BleeperPanelProps {
  /** Bleeper scharf geschaltet */
  initialArmed?: boolean;
  /** Initialer Modus */
  initialMode?: BleeperMode;
  /** Initialer Tone (Hz, für Beep-Modus) */
  initialTone?: number;
  /** Initiale Volume (dB) */
  initialVolume?: number;
}

const MODES: { id: BleeperMode; label: string; icon: string; color: string }[] = [
  { id: 'Beep', label: 'BEEP', icon: '🔊', color: '#ff1744' },
  { id: 'Mute', label: 'MUTE', icon: '🔇', color: '#666' },
  { id: 'Noise', label: 'NOISE', icon: '📻', color: '#e6a117' },
  { id: 'Reverse', label: 'REVERSE', icon: '⏪', color: '#00e5ff' },
  { id: 'Custom', label: 'CUSTOM', icon: '🎵', color: '#ff8c00' },
];

function BleeperPanel({
  initialArmed = false,
  initialMode = 'Beep',
  initialTone = 1000,
  initialVolume = -6,
}: BleeperPanelProps) {
  const [armed, setArmed] = useState(initialArmed);
  const [mode, setMode] = useState<BleeperMode>(initialMode);
  const [tone, setTone] = useState(initialTone);
  const [volume, setVolume] = useState(initialVolume);

  const handleArmToggle = async () => {
    const newArmed = !armed;
    setArmed(newArmed);

    try {
      await invoke('set_bleeper_armed', { armed: newArmed });
    } catch (err) {
      console.error('Fehler beim Setzen des Bleeper-Arm-Status:', err);
    }
  };

  const handleModeClick = async (newMode: BleeperMode) => {
    setMode(newMode);

    try {
      await invoke('set_bleeper_mode', { mode: newMode });
    } catch (err) {
      console.error('Fehler beim Setzen des Bleeper-Modus:', err);
    }
  };

  const handleToneChange = async (value: number) => {
    // 200Hz bis 2000Hz
    const hzValue = 200 + value * 1800;
    setTone(hzValue);

    try {
      await invoke('set_bleeper_tone', { toneHz: hzValue });
    } catch (err) {
      console.error('Fehler beim Setzen des Bleeper-Tons:', err);
    }
  };

  const handleVolumeChange = async (value: number) => {
    // -30dB bis 0dB
    const dbValue = value * -30;
    setVolume(dbValue);

    try {
      await invoke('set_bleeper_volume', { volumeDb: dbValue });
    } catch (err) {
      console.error('Fehler beim Setzen der Bleeper-Lautstärke:', err);
    }
  };

  return (
    <div className="p-3 border-b border-[rgba(255,255,255,0.05)]">
      {/* Header mit Arm-Toggle */}
      <div className="mb-3 flex items-center justify-between">
        <div>
          <h3
            className="text-[7px] font-extrabold uppercase tracking-wider"
            style={{ color: armed ? '#ff1744' : '#666' }}
          >
            Bleeper
          </h3>
          <p className="text-[5px] text-gray-600 mt-0.5">
            Profanity Filter
          </p>
        </div>

        {/* Arm-Toggle (Rot wenn aktiv) */}
        <button
          onClick={handleArmToggle}
          className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-all ${
            armed
              ? 'bg-inox-error text-white shadow-lg shadow-red-500/50'
              : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
          }`}
        >
          {armed ? '⚠ ARMED' : 'DISARMED'}
        </button>
      </div>

      {/* Modus-Buttons */}
      <div className="grid grid-cols-5 gap-1 mb-3">
        {MODES.map((modeOption) => {
          const isActive = mode === modeOption.id;

          return (
            <button
              key={modeOption.id}
              onClick={() => handleModeClick(modeOption.id)}
              disabled={!armed}
              className={`px-1.5 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-all ${
                isActive
                  ? 'bg-gray-700 border-2 shadow-lg'
                  : 'bg-gray-800 border border-gray-700 hover:bg-gray-750'
              }`}
              style={{
                borderColor: isActive ? modeOption.color : undefined,
                color: isActive ? modeOption.color : '#9ca3af',
                opacity: armed ? 1 : 0.4,
              }}
            >
              <div className="text-[10px] mb-0.5">{modeOption.icon}</div>
              <div className="text-[4px]">{modeOption.label}</div>
            </button>
          );
        })}
      </div>

      {/* Parameter Sliders */}
      <div className="space-y-3">
        {/* Tone (nur für Beep-Modus) */}
        {mode === 'Beep' && (
          <Slider
            label="TONE"
            value={(tone - 200) / 1800}
            onChange={handleToneChange}
            color="#ff1744"
            unit="Hz"
            disabled={!armed}
          />
        )}

        {/* Volume (nicht für Mute-Modus) */}
        {mode !== 'Mute' && (
          <Slider
            label="VOLUME"
            value={Math.abs(volume) / 30}
            onChange={handleVolumeChange}
            color="#ff1744"
            unit="dB"
            disabled={!armed}
          />
        )}
      </div>

      {/* Warnung */}
      {armed && (
        <div className="mt-3 p-2 bg-red-500/10 border border-red-500/30 rounded text-center">
          <p className="text-[5px] text-red-400 font-medium">
            ⚠ Bleeper ist scharf! Erkannte Wörter werden automatisch zensiert.
          </p>
        </div>
      )}
    </div>
  );
}

export default BleeperPanel;
