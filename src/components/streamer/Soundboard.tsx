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
    console.log('Add sound dialog');
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
    <div className="p-3">
      {/* Header */}
      <div className="mb-3 flex items-center justify-between">
        <div>
          <h3 className="text-[7px] font-extrabold uppercase tracking-wider text-orange-500">
            Soundboard
          </h3>
          <p className="text-[5px] text-gray-600 mt-0.5">
            {sounds.length} Sounds geladen
          </p>
        </div>

        {/* Add Sound Button */}
        <button
          onClick={handleAddSound}
          className="px-2 py-1 text-[5px] font-bold uppercase tracking-wide bg-orange-500/20 hover:bg-orange-500/30 text-orange-500 border border-orange-500/50 rounded transition-colors"
        >
          + Sound
        </button>
      </div>

      {/* Sound-Pads Grid */}
      <div className="grid grid-cols-2 gap-1.5 mb-3">
        {sounds.length === 0 ? (
          <div className="col-span-2 p-4 text-center text-[5px] text-gray-600">
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
                className={`relative px-2 py-2 text-[5px] font-bold uppercase tracking-wide rounded transition-all ${
                  isPlaying
                    ? 'bg-orange-500 text-white shadow-lg shadow-orange-500/50'
                    : 'bg-gray-800 text-gray-400 hover:bg-gray-700 border border-gray-700'
                }`}
              >
                {/* Sound Name */}
                <div className="text-[5px] mb-1 truncate">{sound.name}</div>

                {/* Hotkey (if set) */}
                {sound.hotkey && (
                  <div className="text-[4px] text-orange-500 font-mono">
                    {sound.hotkey}
                  </div>
                )}

                {/* Playing Indicator */}
                {isPlaying && (
                  <div className="absolute top-1 right-1 w-1.5 h-1.5 rounded-full bg-white animate-pulse" />
                )}
              </button>
            );
          })
        )}
      </div>

      {/* Master Volume */}
      <div className="mt-3 pt-3 border-t border-gray-800">
        <Slider
          label="MASTER VOL"
          value={(masterVolume + 30) / 40}
          onChange={handleMasterVolumeChange}
          color="#ff8c00"
          unit="dB"
        />
      </div>

      {/* Info */}
      <div className="mt-2 text-[4.5px] text-gray-600 text-center">
        Linksklick = Play · Rechtsklick = Stop
      </div>
    </div>
  );
}

export default Soundboard;
