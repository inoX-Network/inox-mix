// Component: MasterSection — Master-Ausgangsstufe (Volume, Limiter, VU, DIM, MONO, TALK)
import { useEffect, useState, useCallback } from 'react';
import { useMasterStore } from '../../stores/masterStore';
import Knob from '../mixer/Knob';
import VUMeter from '../mixer/VUMeter';

const FADER_HEIGHT = 120;
const MIN_DB = -80;
const MAX_DB = 12;

/** dB → Pixel */
function dbToPosition(db: number): number {
  const normalized = (db - MIN_DB) / (MAX_DB - MIN_DB);
  return FADER_HEIGHT - normalized * FADER_HEIGHT;
}

/** Pixel → dB */
function positionToDb(pos: number): number {
  const normalized = 1 - pos / FADER_HEIGHT;
  return MIN_DB + normalized * (MAX_DB - MIN_DB);
}

/** Master Fader: 120px, gleicher Thumb-Style wie Strip */
function MasterFader({ value, onChange }: { value: number; onChange: (v: number) => void }) {
  const [isDragging, setIsDragging] = useState(false);
  const thumbY = dbToPosition(value);

  const updateValue = useCallback((clientY: number) => {
    const rect = document.getElementById('master-fader-track')?.getBoundingClientRect();
    if (!rect) return;
    const y = Math.max(0, Math.min(FADER_HEIGHT, clientY - rect.top));
    onChange(Math.max(MIN_DB, Math.min(MAX_DB, positionToDb(y))));
  }, [onChange]);

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    updateValue(e.clientY);
  };

  useEffect(() => {
    if (!isDragging) return;
    const handleMouseMove = (e: MouseEvent) => updateValue(e.clientY);
    const handleMouseUp = () => setIsDragging(false);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, updateValue]);

  return (
    <div className="relative" style={{ width: '16px', height: `${FADER_HEIGHT}px` }}>
      <div
        id="master-fader-track"
        className="absolute left-1/2 -translate-x-1/2 cursor-pointer rounded-sm"
        style={{ width: '2px', height: `${FADER_HEIGHT - 6}px`, top: '3px', backgroundColor: 'rgba(255,255,255,0.03)' }}
        onMouseDown={handleMouseDown}
      >
        <div
          className="absolute left-0 bottom-0 w-full rounded-sm"
          style={{
            height: `${((value - MIN_DB) / (MAX_DB - MIN_DB)) * (FADER_HEIGHT - 6)}px`,
            backgroundColor: '#00e5ff',
            boxShadow: '0 0 4px #00e5ff25',
          }}
        />
      </div>
      {/* Thumb: 14×9px mit Grip-Lines */}
      <div
        className="absolute left-1/2 -translate-x-1/2 rounded-sm cursor-grab active:cursor-grabbing flex flex-col items-center justify-center gap-[1px]"
        style={{
          width: '14px', height: '9px',
          top: `${thumbY}px`, marginTop: '-4.5px',
          background: 'linear-gradient(180deg, #484848, #282828)',
          border: '1px solid #555',
          boxShadow: '0 1px 2px rgba(0,0,0,0.5)',
        }}
        onMouseDown={handleMouseDown}
      >
        <div style={{ width: '8px', height: '1px', background: 'rgba(255,255,255,0.15)' }} />
        <div style={{ width: '8px', height: '1px', background: 'rgba(255,255,255,0.15)' }} />
        <div style={{ width: '8px', height: '1px', background: 'rgba(255,255,255,0.15)' }} />
      </div>
    </div>
  );
}

/**
 * Master-Sektion — Spec: Label 10px/800/3px tracking, VOL+LIM 22px nebeneinander,
 * VU 100px, Fader 120px, dB 9px, Chips 5px
 */
export default function MasterSection() {
  const {
    volume_db, limiter_ceiling_db, dim, mono, talkback,
    loadMaster, setVolume, setLimiter, setDim, setMono, setTalkback,
  } = useMasterStore();

  const [vuPeakL, setVuPeakL] = useState(-12);
  const [vuPeakR, setVuPeakR] = useState(-10);

  useEffect(() => {
    loadMaster();
    const interval = setInterval(() => {
      setVuPeakL(-20 + Math.random() * 20);
      setVuPeakR(-20 + Math.random() * 20);
    }, 100);
    return () => clearInterval(interval);
  }, [loadMaster]);

  const effectiveDb = dim ? volume_db - 20 : volume_db;

  return (
    <div
      className="flex flex-col items-center gap-[3px] p-[5px] rounded-[5px] border relative"
      style={{
        background: 'linear-gradient(135deg, rgba(0,229,255,0.04), rgba(0,229,255,0.01))',
        borderColor: 'rgba(0,229,255,0.08)',
        boxShadow: '0 0 12px rgba(0,229,255,0.06)',
      }}
    >
      {/* Dock Handle */}
      <div
        className="absolute top-[2px] right-[2px] flex flex-wrap gap-[1px] opacity-0 hover:opacity-30 transition-opacity cursor-grab"
        style={{ width: '8px', height: '8px', alignContent: 'center', justifyContent: 'center' }}
      >
        {Array.from({ length: 6 }).map((_, i) => (
          <i key={i} className="block" style={{ width: '2px', height: '2px', background: 'rgba(255,255,255,0.4)', borderRadius: '50%' }} />
        ))}
      </div>

      {/* Label: 10px, 800 weight, tracking 3px (Mockup: 7px text mit 800 weight im Master-Label) */}
      <div
        style={{
          fontSize: '7px',
          fontWeight: 800,
          color: '#00e5ff',
          letterSpacing: '2px',
          textTransform: 'uppercase',
        }}
      >
        MASTER
      </div>

      {/* VOL + LIM Knobs nebeneinander: 22px */}
      <div className="flex gap-[4px]">
        <Knob value={volume_db} onChange={setVolume} label="VOL" color="cyan" min={-80} max={12} size={22} />
        <Knob value={limiter_ceiling_db} onChange={setLimiter} label="LIM" color="cyan" min={-20} max={0} size={22} />
      </div>

      {/* Dual VU-Meter + Fader */}
      <div className="flex gap-[2px] items-center">
        <VUMeter peak={vuPeakL} rms={vuPeakL - 3} color="cyan" height={100} />
        <MasterFader value={volume_db} onChange={setVolume} />
        <VUMeter peak={vuPeakR} rms={vuPeakR - 3} color="cyan" height={100} />
      </div>

      {/* dB-Anzeige: 9px, Cyan, Bold */}
      <div style={{ fontSize: '9px', fontWeight: 700, color: '#00e5ff' }}>
        {effectiveDb.toFixed(1)} <span style={{ fontSize: '5px', opacity: 0.4 }}>dB</span>
      </div>

      {/* Chips: DIM (Orange bg), MONO, TALK — Spec: 5px, 700, padding 1.5px 4px */}
      <div className="flex flex-wrap gap-[2px] justify-center" style={{ maxWidth: '65px' }}>
        <button
          onClick={() => setDim(!dim)}
          style={{
            padding: '1.5px 4px',
            fontSize: '5px',
            fontWeight: 700,
            letterSpacing: '0.4px',
            textTransform: 'uppercase',
            borderRadius: '2px',
            border: dim ? '1px solid transparent' : '1px solid rgba(255,255,255,0.05)',
            background: dim ? '#ff8c00' : 'rgba(255,255,255,0.01)',
            color: dim ? '#000' : 'rgba(255,255,255,0.18)',
          }}
        >
          DIM
        </button>
        <button
          onClick={() => setMono(!mono)}
          style={{
            padding: '1.5px 4px',
            fontSize: '5px',
            fontWeight: 700,
            letterSpacing: '0.4px',
            textTransform: 'uppercase',
            borderRadius: '2px',
            border: mono ? '1px solid transparent' : '1px solid rgba(255,255,255,0.05)',
            background: mono ? '#00e5ff' : 'rgba(255,255,255,0.01)',
            color: mono ? '#000' : 'rgba(255,255,255,0.18)',
          }}
        >
          MONO
        </button>
        <button
          onClick={() => setTalkback(!talkback, talkback ? [] : ['A1', 'B1'])}
          style={{
            padding: '1.5px 4px',
            fontSize: '5px',
            fontWeight: 700,
            letterSpacing: '0.4px',
            textTransform: 'uppercase',
            borderRadius: '2px',
            border: talkback ? '1px solid transparent' : '1px solid rgba(255,255,255,0.05)',
            background: talkback ? '#00e5ff' : 'rgba(255,255,255,0.01)',
            color: talkback ? '#000' : 'rgba(255,255,255,0.18)',
          }}
        >
          TALK
        </button>
      </div>
    </div>
  );
}
