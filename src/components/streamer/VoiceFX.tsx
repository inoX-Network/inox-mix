// Komponente: VoiceFX — Stimm-Effekte Auswahl (Robot, Vader, etc.)
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Slider from '../mixer/Slider';

/** Voice FX Preset-Namen (synchron mit Backend) */
type VoiceFxPreset = 'None' | 'Robot' | 'Vader' | 'Chipmunk' | 'Megaphone' | 'Echo' | 'Radio';

/** Voice FX Panel mit Effekt-Buttons und Intensitäts-Regler */
interface VoiceFXProps {
  /** Aktiver Effekt */
  initialPreset?: VoiceFxPreset;
  /** Initiale Dry/Wet Mix */
  initialDryWet?: number;
}

const PRESETS: { id: VoiceFxPreset; label: string; icon: string; color: string }[] = [
  { id: 'None', label: 'OFF', icon: '⊘', color: '#666' },
  { id: 'Robot', label: 'ROBOT', icon: '🤖', color: '#00e5ff' },
  { id: 'Vader', label: 'VADER', icon: '👹', color: '#ff1744' },
  { id: 'Chipmunk', label: 'CHIPMUNK', icon: '🐿️', color: '#4caf50' },
  { id: 'Megaphone', label: 'MEGAPHONE', icon: '📢', color: '#e6a117' },
  { id: 'Echo', label: 'ECHO', icon: '🔊', color: '#00e5ff' },
  { id: 'Radio', label: 'RADIO', icon: '📻', color: '#ff8c00' },
];

function VoiceFX({
  initialPreset = 'None',
  initialDryWet = 1.0,
}: VoiceFXProps) {
  const [activePreset, setActivePreset] = useState<VoiceFxPreset>(initialPreset);
  const [dryWet, setDryWet] = useState(initialDryWet);
  const [enabled, setEnabled] = useState(initialPreset !== 'None');

  const handlePresetClick = async (preset: VoiceFxPreset) => {
    setActivePreset(preset);
    const isEnabled = preset !== 'None';
    setEnabled(isEnabled);

    try {
      await invoke('set_voice_fx_preset', { preset });
      await invoke('set_voice_fx_enabled', { enabled: isEnabled });
    } catch (err) {
      console.error('Fehler beim Setzen des Voice FX Presets:', err);
    }
  };

  const handleDryWetChange = async (value: number) => {
    setDryWet(value);

    try {
      await invoke('set_voice_fx_drywet', { dryWet: value });
    } catch (err) {
      console.error('Fehler beim Setzen des Dry/Wet Mix:', err);
    }
  };

  return (
    <div className="p-3 border-b border-[rgba(255,255,255,0.05)]">
      {/* Header */}
      <div className="mb-3 flex items-center justify-between">
        <div>
          <h3 className="text-[7px] font-extrabold uppercase tracking-wider text-cyan-500">
            Voice FX
          </h3>
          <p className="text-[5px] text-gray-600 mt-0.5">
            Stimm-Effekte
          </p>
        </div>

        {/* Status Indicator */}
        {enabled && (
          <div className="flex items-center gap-1">
            <div className="w-1.5 h-1.5 rounded-full bg-cyan-500 animate-pulse" />
            <span className="text-[5px] text-cyan-500 font-medium uppercase">
              Active
            </span>
          </div>
        )}
      </div>

      {/* Preset-Buttons Grid */}
      <div className="grid grid-cols-3 gap-1.5 mb-3">
        {PRESETS.map((preset) => {
          const isActive = activePreset === preset.id;

          return (
            <button
              key={preset.id}
              onClick={() => handlePresetClick(preset.id)}
              className={`px-2 py-2 text-[5px] font-bold uppercase tracking-wide rounded transition-all ${
                isActive
                  ? 'bg-gray-700 border-2 shadow-lg'
                  : 'bg-gray-800 border border-gray-700 hover:bg-gray-750'
              }`}
              style={{
                borderColor: isActive ? preset.color : undefined,
                color: isActive ? preset.color : '#9ca3af',
              }}
            >
              <div className="text-[12px] mb-0.5">{preset.icon}</div>
              <div className="text-[4.5px]">{preset.label}</div>
            </button>
          );
        })}
      </div>

      {/* Dry/Wet Mix Slider */}
      <div className="mt-3">
        <Slider
          label="DRY/WET MIX"
          value={dryWet}
          onChange={handleDryWetChange}
          color="#00e5ff"
          unit="%"
          disabled={!enabled}
        />
      </div>

      {/* Info Text */}
      {enabled && (
        <p className="mt-2 text-[5px] text-gray-500 text-center">
          {activePreset === 'Robot' && 'Pitch-Quantisierung + Ring-Modulator'}
          {activePreset === 'Vader' && 'Tiefe Stimme + Formant-Shift + Hall'}
          {activePreset === 'Chipmunk' && 'Hohe Stimme + Formant-Shift'}
          {activePreset === 'Megaphone' && 'Bandpass + Verzerrung'}
          {activePreset === 'Echo' && 'Hall + Delay'}
          {activePreset === 'Radio' && 'Bandpass + Kompression + Rauschen'}
        </p>
      )}
    </div>
  );
}

export default VoiceFX;
