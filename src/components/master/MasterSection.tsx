// Component: MasterSection — Master-Ausgangsstufe (Volume, Limiter, VU, DIM, MONO, TALK)
import { useEffect, useState } from 'react';
import { useMasterStore } from '../../stores/masterStore';
import Knob from '../mixer/Knob';
import VUMeter from '../mixer/VUMeter';

const FADER_HEIGHT = 120; // px (SPEC: 120px vertikal)
const MIN_DB = -80;
const MAX_DB = 12;

/**
 * dB-Wert in Pixel-Position umrechnen (0 = oben = +12dB, 120 = unten = -80dB)
 */
function dbToPosition(db: number): number {
  const normalized = (db - MIN_DB) / (MAX_DB - MIN_DB); // 0..1
  return FADER_HEIGHT - normalized * FADER_HEIGHT;
}

/**
 * Pixel-Position in dB-Wert umrechnen
 */
function positionToDb(pos: number): number {
  const normalized = 1 - pos / FADER_HEIGHT; // 0..1 (unten→oben)
  return MIN_DB + normalized * (MAX_DB - MIN_DB);
}

/**
 * Master-Fader (vertikal, 120px)
 */
function MasterFader({ value, onChange }: { value: number; onChange: (v: number) => void }) {
  const [isDragging, setIsDragging] = useState(false);
  const thumbY = dbToPosition(value);

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    updateValue(e.clientY);
  };

  const updateValue = (clientY: number) => {
    const rect = document.getElementById('master-fader-track')?.getBoundingClientRect();
    if (!rect) return;
    const y = Math.max(0, Math.min(FADER_HEIGHT, clientY - rect.top));
    const newDb = positionToDb(y);
    onChange(Math.max(MIN_DB, Math.min(MAX_DB, newDb)));
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
  }, [isDragging]);

  return (
    <div className="relative w-[10px]" style={{ height: `${FADER_HEIGHT}px` }}>
      {/* Track */}
      <div
        id="master-fader-track"
        className="absolute left-1/2 top-0 w-[2px] h-full bg-inox-subtle/20 -translate-x-1/2 cursor-pointer"
        onMouseDown={handleMouseDown}
      />
      {/* Thumb */}
      <div
        className="absolute left-1/2 w-[10px] h-[9px] rounded-[2px] bg-inox-cyan -translate-x-1/2 cursor-grab active:cursor-grabbing"
        style={{ top: `${thumbY}px` }}
        onMouseDown={handleMouseDown}
      />
    </div>
  );
}

/**
 * Master-Sektion: Volume, Limiter, VU-Meter, Fader, DIM, MONO, Talkback
 */
export default function MasterSection() {
  const {
    volume_db,
    limiter_ceiling_db,
    dim,
    mono,
    talkback,
    loadMaster,
    setVolume,
    setLimiter,
    setDim,
    setMono,
    setTalkback,
  } = useMasterStore();

  // Dummy VU-Meter Werte (später aus Backend)
  const [vuPeakL, setVuPeakL] = useState(-12);
  const [vuPeakR, setVuPeakR] = useState(-10);

  useEffect(() => {
    loadMaster();

    // Dummy VU-Animation
    const interval = setInterval(() => {
      setVuPeakL(-20 + Math.random() * 20);
      setVuPeakR(-20 + Math.random() * 20);
    }, 100);

    return () => clearInterval(interval);
  }, [loadMaster]);

  // Effektive Lautstärke (mit DIM)
  const effectiveDb = dim ? volume_db - 20 : volume_db;

  return (
    <div
      className="flex flex-col items-center gap-3 p-4 rounded-[5px] border"
      style={{
        background: 'linear-gradient(135deg, rgba(0,229,255,0.04), rgba(0,229,255,0.01))',
        borderColor: 'rgba(0,229,255,0.08)',
        minWidth: '140px',
      }}
    >
      {/* Label */}
      <div className="text-[10px] font-bold text-inox-cyan tracking-[3px] uppercase">
        MASTER
      </div>

      {/* VOL Knob */}
      <div className="flex flex-col items-center gap-1">
        <Knob
          value={volume_db}
          onChange={setVolume}
          label="VOL"
          color="cyan"
          min={-80}
          max={12}
          size={24}
        />
      </div>

      {/* LIM Knob */}
      <div className="flex flex-col items-center gap-1">
        <Knob
          value={limiter_ceiling_db}
          onChange={setLimiter}
          label="LIM"
          color="orange"
          min={-20}
          max={0}
          size={24}
        />
      </div>

      {/* Dual VU-Meter (L/R) */}
      <div className="flex gap-1 h-[60px]">
        <div className="w-[4px]">
          <VUMeter peak={vuPeakL} rms={vuPeakL - 3} color="cyan" />
        </div>
        <div className="w-[4px]">
          <VUMeter peak={vuPeakR} rms={vuPeakR - 3} color="cyan" />
        </div>
      </div>

      {/* Master Fader */}
      <div className="flex flex-col items-center gap-2">
        <MasterFader value={volume_db} onChange={setVolume} />
      </div>

      {/* dB-Anzeige */}
      <div className="text-[11px] font-bold text-inox-cyan">
        {effectiveDb.toFixed(1)} dB
      </div>

      {/* Chips: DIM, MONO, TALK */}
      <div className="flex flex-col gap-1 w-full">
        {/* DIM (Orange) */}
        <button
          onClick={() => setDim(!dim)}
          className={`px-2 py-1 rounded-[3px] text-[7px] font-bold tracking-wider uppercase transition-colors ${
            dim
              ? 'bg-inox-orange text-inox-bg'
              : 'bg-inox-panel text-inox-muted hover:text-inox-text border border-inox-subtle/20'
          }`}
        >
          DIM
        </button>

        {/* MONO */}
        <button
          onClick={() => setMono(!mono)}
          className={`px-2 py-1 rounded-[3px] text-[7px] font-bold tracking-wider uppercase transition-colors ${
            mono
              ? 'bg-inox-cyan text-inox-bg'
              : 'bg-inox-panel text-inox-muted hover:text-inox-text border border-inox-subtle/20'
          }`}
        >
          MONO
        </button>

        {/* TALK */}
        <button
          onClick={() => setTalkback(!talkback, talkback ? [] : ['A1', 'B1'])}
          className={`px-2 py-1 rounded-[3px] text-[7px] font-bold tracking-wider uppercase transition-colors ${
            talkback
              ? 'bg-inox-cyan text-inox-bg'
              : 'bg-inox-panel text-inox-muted hover:text-inox-text border border-inox-subtle/20'
          }`}
        >
          TALK
        </button>
      </div>
    </div>
  );
}
