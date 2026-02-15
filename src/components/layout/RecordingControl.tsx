// Komponente: RecordingControl — Recording Start/Stop im Header
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface ActiveRecording {
  source_id: string;
  format: 'Wav' | 'Flac';
  start_time: number;
}

/** Recording-Control mit REC-Button und Status */
export default function RecordingControl() {
  const [recording, setRecording] = useState(false);
  const [format, setFormat] = useState<'Wav' | 'Flac'>('Flac');
  const [activeRecordings, setActiveRecordings] = useState<ActiveRecording[]>([]);

  // Aktive Aufnahmen laden
  useEffect(() => {
    const loadStatus = () => {
      invoke<ActiveRecording[]>('get_recording_status')
        .then(setActiveRecordings)
        .catch(console.error);
    };

    loadStatus();
    const interval = setInterval(loadStatus, 1000); // Alle 1s aktualisieren

    return () => clearInterval(interval);
  }, []);

  // Recording starten/stoppen
  const toggleRecording = () => {
    if (recording) {
      // Stop: Alle aktiven Aufnahmen stoppen
      activeRecordings.forEach((rec) => {
        invoke('stop_recording', { sourceId: rec.source_id })
          .catch(console.error);
      });
      setRecording(false);
    } else {
      // Start: Master-Out aufnehmen
      invoke('start_recording', { sourceId: 'master_out', format })
        .then(() => setRecording(true))
        .catch(console.error);
    }
  };

  const isRecording = activeRecordings.length > 0;

  return (
    <div className="flex items-center gap-2">
      {/* Format-Dropdown */}
      <select
        className="bg-inox-strip border border-inox-subtle/20 rounded text-[6px] text-inox-text px-1 py-0.5 font-mono"
        value={format}
        onChange={(e) => setFormat(e.target.value as 'Wav' | 'Flac')}
        disabled={isRecording}
      >
        <option value="Wav">WAV</option>
        <option value="Flac">FLAC</option>
      </select>

      {/* REC-Button */}
      <button
        onClick={toggleRecording}
        className={`text-[6px] font-bold uppercase tracking-[1px] px-2 py-0.5 rounded border transition-colors ${
          isRecording
            ? 'border-inox-red text-inox-red bg-inox-red/10 animate-pulse'
            : 'border-inox-subtle/20 text-inox-muted hover:text-inox-dim hover:border-inox-subtle/40'
        }`}
      >
        {isRecording ? '⏹ REC' : '⏺ REC'}
      </button>

      {/* Status-Indicator */}
      {isRecording && (
        <div className="w-[4px] h-[4px] rounded-full bg-inox-red shadow-[0_0_3px_rgba(255,23,68,0.8)] animate-pulse" />
      )}
    </div>
  );
}
