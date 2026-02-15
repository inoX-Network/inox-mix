// Komponente: OutputBus — Ausgangs-Bus Strip (A1 Speakers, A2 Headset, B1 Stream, B2 VoIP)
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import VUMeter from '../mixer/VUMeter';
import Slider from '../mixer/Slider';
import { useStripMetering } from '../../hooks/useMetering';

/** Output Bus Strip mit Fader, VU-Meter und Geräte-Auswahl */
interface OutputBusProps {
  /** Bus-ID (z.B. "A1", "B1") */
  busId: string;
  /** Bus-Name (z.B. "SPEAKERS", "STREAM") */
  name: string;
  /** Bus-Typ: "A" (Cyan, physisch) oder "B" (Orange, virtuell) */
  type: 'A' | 'B';
  /** Initiale Lautstärke in dB */
  initialVolume?: number;
  /** Initialer Mute-Status */
  initialMuted?: boolean;
  /** Initiales Recording */
  initialRecording?: boolean;
}

const MIN_DB = -50;
const MAX_DB = 10;

/**
 * dB zu 0-1 normalisieren
 */
function dbToNormalized(db: number): number {
  return (db - MIN_DB) / (MAX_DB - MIN_DB);
}

/**
 * 0-1 zu dB konvertieren
 */
function normalizedToDb(value: number): number {
  return MIN_DB + value * (MAX_DB - MIN_DB);
}

function OutputBus({
  busId,
  name,
  type,
  initialVolume = 0,
  initialMuted = false,
  initialRecording = false,
}: OutputBusProps) {
  const [volumeDb, setVolumeDb] = useState(initialVolume);
  const [muted, setMuted] = useState(initialMuted);
  const [recording, setRecording] = useState(initialRecording);

  // Echtzeit-Metering-Daten aus Backend
  const metering = useStripMetering(busId);
  const peak = metering?.peak_l ?? -60;
  const rms = metering?.rms_l ?? -60;

  const color = type === 'A' ? 'cyan' : 'orange';
  const colorHex = type === 'A' ? '#00e5ff' : '#ff8c00';
  const accentOpacity = 0.4;

  const handleVolumeChange = async (value: number) => {
    const newDb = normalizedToDb(value);
    setVolumeDb(newDb);

    try {
      await invoke('set_bus_volume', { busId, volumeDb: newDb });
    } catch (err) {
      console.error('Fehler beim Setzen der Bus-Lautstärke:', err);
    }
  };

  const handleMuteToggle = async () => {
    const newMuted = !muted;
    setMuted(newMuted);

    try {
      await invoke('set_bus_mute', { busId, muted: newMuted });
    } catch (err) {
      console.error('Fehler beim Setzen des Bus-Mute:', err);
    }
  };

  const handleRecToggle = () => {
    setRecording(!recording);
    // TODO: Recording-Logik
  };

  // Vermeidung wissenschaftlicher Notation und saubere dB-Anzeige
  const displayDb = volumeDb >= 0
    ? `+${volumeDb.toFixed(1)}`
    : volumeDb.toFixed(1);

  return (
    <div
      className="relative bg-inox-strip border border-[rgba(255,255,255,0.05)] rounded-[5px] p-3 min-w-[180px] flex flex-col gap-2"
      style={{
        // Top-Accent: 2px Bus-Farbe
        borderTop: `2px solid ${colorHex}`,
        borderTopColor: `${colorHex}${Math.round(accentOpacity * 255).toString(16).padStart(2, '0')}`,
      }}
    >
      {/* Header: Bus-ID + Name */}
      <div className="flex items-baseline gap-2">
        <span
          className="text-[9px] font-extrabold uppercase tracking-wider"
          style={{ color: colorHex, letterSpacing: '1px' }}
        >
          {busId}
        </span>
        <span className="text-[6px] font-medium uppercase text-gray-500 tracking-wide">
          {name}
        </span>
      </div>

      {/* Geräte-Auswahl Dropdown (nur für physische Busse A1/A2) */}
      {type === 'A' && (
        <select
          className="w-full text-[6px] font-medium bg-inox-panel border border-gray-700 rounded px-2 py-1 text-gray-300 focus:outline-none focus:border-gray-600"
          style={{ fontSize: '6px' }}
        >
          <option>Standard Ausgabe</option>
          <option>USB Headset</option>
          <option>HDMI Audio</option>
          {/* TODO: Dynamische Geräteliste aus PipeWire */}
        </select>
      )}

      {/* Volume Slider + dB-Anzeige */}
      <div className="flex items-center gap-3">
        {/* VU-Meter (Stereo, kompakt) */}
        <div className="flex gap-[2px] h-[40px]">
          <VUMeter peak={peak} rms={rms} color={color} />
          <VUMeter peak={peak} rms={rms} color={color} />
        </div>

        {/* Volume Slider (horizontal) */}
        <div className="flex-1">
          <Slider
            label="VOL"
            value={dbToNormalized(volumeDb)}
            onChange={handleVolumeChange}
            color={colorHex}
            unit="dB"
            disabled={muted}
          />
        </div>

        {/* dB-Anzeige */}
        <div
          className="text-[7px] font-bold tabular-nums min-w-[28px] text-right"
          style={{ color: muted ? '#666' : colorHex }}
        >
          {displayDb} dB
        </div>
      </div>

      {/* Buttons: MUTE + REC */}
      <div className="flex gap-2">
        {/* MUTE Button */}
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

        {/* REC Button */}
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

export default OutputBus;
