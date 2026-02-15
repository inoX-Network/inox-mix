// Komponente: StreamMaster — Stream-Fader und Monitor in der Sidebar
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Fader from '../mixer/Fader';
import VUMeter from '../mixer/VUMeter';

/** Stream Master mit Fader, VU-Meter und 4-Wellen Monitor */
interface StreamMasterProps {
  /** Stream-Lautstärke in dB */
  initialVolume?: number;
  /** Initialer Mute-Status */
  initialMuted?: boolean;
  /** Initiales Recording */
  initialRecording?: boolean;
}

function StreamMaster({
  initialVolume = 0,
  initialMuted = false,
  initialRecording = false,
}: StreamMasterProps) {
  const [volumeDb, setVolumeDb] = useState(initialVolume);
  const [muted, setMuted] = useState(initialMuted);
  const [recording, setRecording] = useState(initialRecording);
  const [peak, setPeak] = useState(-60); // TODO: Aus Metering-Hook
  const [rms, setRms] = useState(-60); // TODO: Aus Metering-Hook

  const handleVolumeChange = async (newDb: number) => {
    setVolumeDb(newDb);

    try {
      // Stream ist Bus B1
      await invoke('set_bus_volume', { busId: 'B1', volumeDb: newDb });
    } catch (err) {
      console.error('Fehler beim Setzen der Stream-Lautstärke:', err);
    }
  };

  const handleMuteToggle = async () => {
    const newMuted = !muted;
    setMuted(newMuted);

    try {
      await invoke('set_bus_mute', { busId: 'B1', muted: newMuted });
    } catch (err) {
      console.error('Fehler beim Setzen des Stream-Mute:', err);
    }
  };

  const handleRecToggle = () => {
    setRecording(!recording);
    // TODO: Recording-Logik für Stream
  };

  const displayDb = volumeDb >= 0
    ? `+${volumeDb.toFixed(1)}`
    : volumeDb.toFixed(1);

  return (
    <div className="p-3 border-b border-[rgba(255,255,255,0.05)]">
      {/* Header */}
      <div className="mb-3">
        <h3 className="text-[8px] font-extrabold uppercase tracking-wider text-orange-500">
          Stream Master
        </h3>
        <p className="text-[5px] text-gray-600 mt-0.5">
          Bus B1 → Stream Output
        </p>
      </div>

      {/* Fader + VU-Meter Layout */}
      <div className="flex items-center gap-3">
        {/* VU-Meter Links (Stereo) */}
        <div className="flex gap-[2px] h-[90px]">
          <VUMeter peak={peak} rms={rms} color="orange" />
          <VUMeter peak={peak} rms={rms} color="orange" />
        </div>

        {/* Fader Mitte */}
        <Fader
          value={volumeDb}
          onChange={handleVolumeChange}
          color="orange"
          disabled={muted}
        />

        {/* VU-Meter Rechts (Stereo) */}
        <div className="flex gap-[2px] h-[90px]">
          <VUMeter peak={peak} rms={rms} color="orange" />
          <VUMeter peak={peak} rms={rms} color="orange" />
        </div>
      </div>

      {/* dB-Anzeige */}
      <div className="mt-2 text-center">
        <span
          className="text-[8px] font-bold tabular-nums"
          style={{ color: muted ? '#666' : '#ff8c00' }}
        >
          {displayDb} dB
        </span>
      </div>

      {/* Buttons */}
      <div className="mt-3 flex gap-2">
        <button
          onClick={handleMuteToggle}
          className={`flex-1 px-2 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
            muted
              ? 'bg-inox-error text-white'
              : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
          }`}
        >
          MUTE
        </button>

        <button
          onClick={handleRecToggle}
          className={`flex-1 px-2 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
            recording
              ? 'bg-inox-error text-white animate-pulse'
              : 'bg-gray-800 text-gray-400 hover:bg-gray-700'
          }`}
        >
          REC
        </button>
      </div>
    </div>
  );
}

export default StreamMaster;
