// Komponente: StreamSidebar — Slide-out Sidebar für Streamer-Features (270px rechts)
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

/** Voice FX State vom Backend */
interface VoiceFxState {
  preset: 'none' | 'robot' | 'vader' | 'chipmunk' | 'megaphone' | 'echo' | 'radio';
  enabled: boolean;
  dry_wet: number;
}

/** Soundboard Entry vom Backend */
interface SoundEntry {
  id: string;
  name: string;
  file_path: string;
  hotkey: string | null;
  bus_id: string;
  volume_db: number;
  created_at: number;
}

/** Streamer-Sidebar mit Voice FX und Soundboard */
function StreamSidebar() {
  const [voiceFx, setVoiceFx] = useState<VoiceFxState>({
    preset: 'none',
    enabled: false,
    dry_wet: 1.0,
  });
  const [sounds, setSounds] = useState<SoundEntry[]>([]);

  // Voice FX State laden
  useEffect(() => {
    invoke<VoiceFxState>('get_voice_fx_state')
      .then(setVoiceFx)
      .catch(console.error);
  }, []);

  // Sounds laden
  useEffect(() => {
    invoke<SoundEntry[]>('get_sounds')
      .then(setSounds)
      .catch(console.error);
  }, []);

  // Voice FX Preset ändern
  const setPreset = (preset: VoiceFxState['preset']) => {
    invoke('set_voice_fx_preset', { preset })
      .then(() => setVoiceFx((s) => ({ ...s, preset })))
      .catch(console.error);
  };

  // Voice FX Enable Toggle
  const toggleVoiceFx = () => {
    const newEnabled = !voiceFx.enabled;
    invoke('set_voice_fx_enabled', { enabled: newEnabled })
      .then(() => setVoiceFx((s) => ({ ...s, enabled: newEnabled })))
      .catch(console.error);
  };

  // Voice FX Dry/Wet ändern
  const setDryWet = (dry_wet: number) => {
    invoke('set_voice_fx_drywet', { dryWet: dry_wet })
      .then(() => setVoiceFx((s) => ({ ...s, dry_wet })))
      .catch(console.error);
  };

  // Sound abspielen
  const playSound = (soundId: string) => {
    invoke('play_sound', { soundId }).catch(console.error);
  };

  return (
    <aside className="w-[270px] bg-inox-panel border-l border-inox-subtle/20 flex flex-col overflow-y-auto">
      {/* Header */}
      <div className="bg-inox-strip border-b border-inox-subtle/20 px-3 py-2 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-2">
          <div className="w-[6px] h-[6px] rounded-full bg-inox-red shadow-[0_0_4px_rgba(255,23,68,0.8)] animate-pulse" />
          <span className="text-[8px] font-bold text-inox-cyan tracking-wider uppercase">
            STREAMER
          </span>
        </div>
        <span className="text-[6px] text-inox-muted font-medium">
          LIVE
        </span>
      </div>

      {/* Voice FX Sektion */}
      <div className="border-b border-inox-subtle/20 p-3">
        <div className="flex items-center justify-between mb-2">
          <span className="text-[7px] font-bold text-inox-orange tracking-wider uppercase">
            VOICE FX
          </span>
          <button
            className={`text-[6px] font-bold px-2 py-0.5 rounded-sm transition-colors ${
              voiceFx.enabled
                ? 'bg-inox-orange/40 text-inox-orange'
                : 'bg-inox-subtle text-inox-muted'
            }`}
            onClick={toggleVoiceFx}
          >
            {voiceFx.enabled ? 'ON' : 'OFF'}
          </button>
        </div>

        {/* Preset Dropdown */}
        <select
          className="w-full bg-inox-strip border border-inox-subtle/20 rounded text-[7px] text-inox-text px-2 py-1 mb-2 font-mono"
          value={voiceFx.preset}
          onChange={(e) => setPreset(e.target.value as VoiceFxState['preset'])}
          disabled={!voiceFx.enabled}
        >
          <option value="none">Aus</option>
          <option value="robot">Robot</option>
          <option value="vader">Vader</option>
          <option value="chipmunk">Chipmunk</option>
          <option value="megaphone">Megaphone</option>
          <option value="echo">Echo</option>
          <option value="radio">Radio</option>
        </select>

        {/* Dry/Wet Slider */}
        <div className="flex flex-col gap-1">
          <span className="text-[6px] text-inox-muted uppercase tracking-wide">
            Dry/Wet Mix
          </span>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={voiceFx.dry_wet}
            onChange={(e) => setDryWet(parseFloat(e.target.value))}
            disabled={!voiceFx.enabled}
            className="w-full h-1 bg-inox-subtle rounded-full appearance-none cursor-pointer accent-inox-orange"
          />
          <span className="text-[6px] text-inox-orange text-center font-mono">
            {(voiceFx.dry_wet * 100).toFixed(0)}%
          </span>
        </div>
      </div>

      {/* Soundboard Sektion */}
      <div className="p-3 flex-1 overflow-y-auto">
        <div className="flex items-center justify-between mb-2">
          <span className="text-[7px] font-bold text-inox-cyan tracking-wider uppercase">
            SOUNDBOARD
          </span>
          <button className="text-[6px] font-bold px-2 py-0.5 bg-inox-cyan/40 text-inox-cyan rounded-sm hover:bg-inox-cyan/60 transition-colors">
            + ADD
          </button>
        </div>

        {/* Sound Pads Grid */}
        <div className="grid grid-cols-2 gap-2">
          {sounds.map((sound) => (
            <button
              key={sound.id}
              onClick={() => playSound(sound.id)}
              className="bg-inox-strip border border-inox-subtle/20 rounded p-2 hover:bg-inox-subtle/20 transition-colors flex flex-col items-center justify-center gap-1"
            >
              <span className="text-[7px] font-bold text-inox-cyan truncate w-full text-center">
                {sound.name}
              </span>
              {sound.hotkey && (
                <span className="text-[6px] text-inox-muted font-mono">
                  {sound.hotkey}
                </span>
              )}
            </button>
          ))}
        </div>

        {/* Keine Sounds */}
        {sounds.length === 0 && (
          <div className="text-[6px] text-inox-muted text-center py-4">
            Keine Sounds hinzugefügt
          </div>
        )}
      </div>
    </aside>
  );
}

export default StreamSidebar;
