// Komponente: Soundboard — Sound-Pad Grid für Stream-Sounds
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Slider from '../mixer/Slider';

/** Sound-Entry aus Backend */
interface SoundEntry {
  id: string;
  name: string;
  file_path: string;
  hotkey: string | null;
  bus_id: string | null;
  volume_db: number;
}

/** Soundboard mit konfigurierbaren Sound-Pads */
interface SoundboardProps {}

function Soundboard(_props: SoundboardProps) {
  const [sounds, setSounds] = useState<SoundEntry[]>([]);
  const [masterVolume, setMasterVolume] = useState(0);
  const [playing, setPlaying] = useState<Set<string>>(new Set());

  useEffect(() => {
    // Sounds vom Backend laden
    loadSounds();
  }, []);

  const loadSounds = async () => {
    try {
      const soundsList = await invoke<SoundEntry[]>('get_sounds');
      setSounds(soundsList);
    } catch (err) {
      console.error('Fehler beim Laden der Sounds:', err);
    }
  };

  const handlePlaySound = async (soundId: string) => {
    setPlaying((prev) => new Set(prev).add(soundId));

    try {
      await invoke('play_sound', { soundId });
    } catch (err) {
      console.error('Fehler beim Abspielen des Sounds:', err);
    }

    // Nach 2 Sekunden aus Playing-Set entfernen (simuliert)
    setTimeout(() => {
      setPlaying((prev) => {
        const next = new Set(prev);
        next.delete(soundId);
        return next;
      });
    }, 2000);
  };

  const handleStopSound = async (soundId: string) => {
    try {
      await invoke('stop_sound', { soundId });
      setPlaying((prev) => {
        const next = new Set(prev);
        next.delete(soundId);
        return next;
      });
    } catch (err) {
      console.error('Fehler beim Stoppen des Sounds:', err);
    }
  };

  const handleAddSound = () => {
    // TODO: Open file dialog to add new sound
  };

  const handleMasterVolumeChange = async (value: number) => {
    // -30dB bis +10dB
    const dbValue = -30 + value * 40;
    setMasterVolume(dbValue);

    try {
      await invoke('set_soundboard_volume', { volumeDb: dbValue });
    } catch (err) {
      console.error('Fehler beim Setzen der Soundboard-Lautstärke:', err);
    }
  };

  return (
    <div className="p-6">
      {/* Header */}
      <div className="mb-6 flex items-center justify-between">
        <div>
          <h3 className="text-[13px] font-extrabold uppercase tracking-wider text-inox-orange">
            Soundboard
          </h3>
          <p className="text-[9px] text-inox-muted mt-1">
            {sounds.length} Sounds geladen
          </p>
        </div>

        {/* Add Sound Button */}
        <button
          onClick={handleAddSound}
          className="px-4 py-2 text-[9px] font-bold uppercase tracking-wide bg-inox-orange/20 hover:bg-inox-orange/30 text-inox-orange border border-inox-orange/50 rounded transition-colors"
        >
          + Sound
        </button>
      </div>

      {/* Sound-Pads Grid */}
      <div className="grid grid-cols-2 gap-4 mb-6">
        {sounds.length === 0 ? (
          <div className="col-span-2 p-8 text-center text-[11px] text-inox-muted">
            Keine Sounds vorhanden. Klicke auf "+ Sound" um einen hinzuzufügen.
          </div>
        ) : (
          sounds.map((sound) => {
            const isPlaying = playing.has(sound.id);

            return (
              <button
                key={sound.id}
                onClick={() => handlePlaySound(sound.id)}
                onContextMenu={(e) => {
                  e.preventDefault();
                  handleStopSound(sound.id);
                }}
                className={`relative px-6 py-6 text-[11px] font-bold uppercase tracking-wide rounded transition-all ${
                  isPlaying
                    ? 'bg-inox-orange text-white shadow-lg shadow-inox-orange/50'
                    : 'bg-inox-strip text-inox-muted hover:bg-inox-subtle border border-inox-subtle'
                }`}
              >
                {/* Sound Name */}
                <div className="text-[13px] mb-3 truncate">{sound.name}</div>

                {/* Hotkey (if set) */}
                {sound.hotkey && (
                  <div className="text-[10px] text-inox-orange font-mono">
                    {sound.hotkey}
                  </div>
                )}

                {/* Playing Indicator */}
                {isPlaying && (
                  <div className="absolute top-3 right-3 w-4 h-4 rounded-full bg-white animate-pulse" />
                )}
              </button>
            );
          })
        )}
      </div>

      {/* Master Volume */}
      <div className="mt-6 pt-6 border-t border-inox-subtle/20">
        <Slider
          label="MASTER VOL"
          value={(masterVolume + 30) / 40}
          onChange={handleMasterVolumeChange}
          color="#ff8c00"
          unit="dB"
        />
      </div>

      {/* Info */}
      <div className="mt-4 text-[8px] text-inox-muted text-center">
        Linksklick = Play · Rechtsklick = Stop
      </div>
    </div>
  );
}

export default Soundboard;
