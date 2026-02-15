// Component: StreamSidebar — Vollständige Stream-Sidebar (270px rechts)
// Spec: 270px wide, padding 10px, shadow -4px 0 24px
import { useState } from 'react';
import Fader from '../mixer/Fader';
import VUMeter from '../mixer/VUMeter';
import StreamMonitor from './StreamMonitor';
import AudioProtection from './AudioProtection';
import VoiceFXTiles from './VoiceFXTiles';

/** Streamer-Sidebar mit allen Features */
export default function StreamSidebar() {
  // Stream Master State (dummy)
  const [streamVolume, setStreamVolume] = useState(-4.2);
  const [streamMuted, setStreamMuted] = useState(false);
  const [streamRecording, setStreamRecording] = useState(false);

  // Dummy Soundboard Sounds
  const sounds = [
    { id: '1', name: 'Airhorn', icon: '📣' },
    { id: '2', name: 'Rimshot', icon: '🥁' },
    { id: '3', name: 'Applaus', icon: '👏' },
    { id: '4', name: 'Fail', icon: '💥' },
  ];

  return (
    <aside
      className="flex flex-col overflow-y-auto overflow-x-hidden"
      style={{
        width: '270px',
        background: 'var(--color-inox-panel)',
        borderLeft: '1px solid rgba(255,109,0,0.1)',
        padding: '10px',
        boxShadow: '-4px 0 24px rgba(0,0,0,0.6)',
      }}
    >
      {/* Header: 📡 STREAMER + "Bus B1 — Stream Output" + LIVE Chip */}
      <div
        className="flex items-center justify-between"
        style={{ marginBottom: '8px', paddingBottom: '6px', borderBottom: '1px solid rgba(255,109,0,0.1)' }}
      >
        <div className="flex items-center gap-[5px]">
          <span style={{ fontSize: '14px' }}>📡</span>
          <div>
            <div style={{ fontSize: '10px', fontWeight: 800, color: '#ff8c00', letterSpacing: '2px', textTransform: 'uppercase' }}>
              STREAMER
            </div>
            <div style={{ fontSize: '5px', color: 'rgba(255,255,255,0.1)' }}>
              Bus B1 — Stream Output
            </div>
          </div>
        </div>
        <span
          style={{
            padding: '3px 8px',
            fontSize: '7px',
            fontWeight: 700,
            letterSpacing: '0.8px',
            textTransform: 'uppercase',
            borderRadius: '2px',
            background: '#ff8c00',
            border: '1px solid #ff8c00',
            color: '#000',
          }}
        >
          LIVE
        </span>
      </div>

      {/* Stream Master + Monitor NEBENEINANDER */}
      <div className="flex gap-[5px]" style={{ marginBottom: '8px' }}>
        {/* LINKS: Stream Master Fader */}
        <div
          className="flex flex-col items-center gap-[3px]"
          style={{
            background: 'rgba(0,0,0,0.25)',
            borderRadius: '5px',
            padding: '6px',
            border: '1px solid rgba(255,109,0,0.08)',
            minWidth: '56px',
          }}
        >
          <div style={{ fontSize: '5.5px', fontWeight: 700, color: '#ff8c00', letterSpacing: '1.5px', textTransform: 'uppercase' }}>
            STREAM
          </div>
          <div className="flex items-center gap-[2px]">
            <VUMeter peak={-8} rms={-12} color="orange" height={70} />
            <Fader value={streamVolume} onChange={setStreamVolume} color="orange" disabled={streamMuted} height={75} />
            <VUMeter peak={-6} rms={-10} color="orange" height={70} />
          </div>
          <div style={{ fontSize: '8px', fontWeight: 700, color: '#ff8c00' }}>
            {streamVolume.toFixed(1)} <span style={{ fontSize: '5px', opacity: 0.4 }}>dB</span>
          </div>
          <div className="flex gap-[2px]">
            <button
              style={{
                padding: '1px 6px',
                fontSize: '5px',
                fontWeight: 700,
                letterSpacing: '0.4px',
                textTransform: 'uppercase',
                borderRadius: '2px',
                border: `1px solid ${streamMuted ? 'rgba(255,23,68,0.1)' : 'rgba(255,255,255,0.05)'}`,
                background: streamMuted ? 'rgba(255,23,68,0.06)' : 'rgba(255,255,255,0.01)',
                color: streamMuted ? '#ff1744' : 'rgba(255,255,255,0.18)',
                cursor: 'pointer',
              }}
              onClick={() => setStreamMuted(!streamMuted)}
            >
              MUTE
            </button>
            <button
              style={{
                padding: '1px 6px',
                fontSize: '5px',
                fontWeight: 700,
                letterSpacing: '0.4px',
                textTransform: 'uppercase',
                borderRadius: '2px',
                border: `1px solid ${streamRecording ? 'rgba(255,23,68,0.1)' : 'rgba(255,255,255,0.05)'}`,
                background: streamRecording ? 'rgba(255,23,68,0.06)' : 'rgba(255,255,255,0.01)',
                color: streamRecording ? '#ff1744' : 'rgba(255,255,255,0.18)',
                cursor: 'pointer',
              }}
              onClick={() => setStreamRecording(!streamRecording)}
            >
              ●REC
            </button>
          </div>
        </div>

        {/* RECHTS: Stream Monitor (4 Wellen) */}
        <StreamMonitor />
      </div>

      {/* AUDIO PROTECTION Box (Ducking + Bleeper kombiniert) */}
      <AudioProtection />

      {/* VOICE FX (Kacheln) */}
      <VoiceFXTiles />

      {/* SOUNDBOARD */}
      <div
        className="rounded-[5px]"
        style={{
          background: 'linear-gradient(135deg, rgba(255,196,0,0.03), rgba(255,109,0,0.03))',
          border: '1px solid rgba(255,196,0,0.06)',
          padding: '8px',
          marginBottom: '5px',
        }}
      >
        {/* Header */}
        <div className="flex items-center justify-between" style={{ marginBottom: '5px' }}>
          <div className="flex items-center gap-[4px]">
            <span style={{ fontSize: '11px' }}>🎹</span>
            <span style={{ fontSize: '7px', fontWeight: 700, letterSpacing: '1.5px', color: '#ff8c00', textTransform: 'uppercase' }}>
              SOUNDBOARD
            </span>
          </div>
        </div>

        {/* Sound-Buttons: 44×34px */}
        <div className="flex gap-[3px] flex-wrap justify-center">
          {sounds.map((sound) => (
            <button
              key={sound.id}
              className="inline-flex flex-col items-center justify-center gap-[1px] transition-all"
              style={{
                width: '44px',
                height: '34px',
                borderRadius: '3px',
                border: '1px solid rgba(255,255,255,0.05)',
                background: 'rgba(255,255,255,0.015)',
                cursor: 'pointer',
              }}
              onMouseEnter={(e) => { e.currentTarget.style.borderColor = 'rgba(255,255,255,0.1)'; }}
              onMouseLeave={(e) => { e.currentTarget.style.borderColor = 'rgba(255,255,255,0.05)'; }}
            >
              <span style={{ fontSize: '11px' }}>{sound.icon}</span>
              <span style={{ fontSize: '5px', color: 'rgba(255,255,255,0.18)' }}>{sound.name}</span>
            </button>
          ))}
        </div>
      </div>
    </aside>
  );
}
