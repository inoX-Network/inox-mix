// Component: Fader — Vertikaler Audio-Fader mit Drag + MouseWheel
import { useState, useRef, useCallback, useEffect } from 'react';

interface FaderProps {
  /** Aktueller Wert in dB (-50 bis +10) */
  value: number;
  /** Callback bei Wert-Änderung */
  onChange: (value: number) => void;
  /** Farbe (cyan für Hardware, orange für Virtual) */
  color: 'cyan' | 'orange';
  /** Deaktiviert (muted) */
  disabled?: boolean;
  /** Fader-Höhe in px (default: 90 für Strip, 120 für Master) */
  height?: number;
}

const MIN_DB = -50;
const MAX_DB = 10;

/** dB-Wert in Pixel-Position umrechnen */
function dbToPosition(db: number, h: number): number {
  const normalized = (db - MIN_DB) / (MAX_DB - MIN_DB);
  return h - normalized * h;
}

/** Pixel-Position in dB-Wert umrechnen */
function positionToDb(pos: number, h: number): number {
  const normalized = 1 - pos / h;
  return MIN_DB + normalized * (MAX_DB - MIN_DB);
}

/** Vertikaler Fader — Spec: Track 2px, Thumb 14×9px, 3 Grip-Lines */
export default function Fader({ value, onChange, color, disabled = false, height = 90 }: FaderProps) {
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);
  const thumbY = dbToPosition(value, height);
  const fillColor = color === 'cyan' ? '#00e5ff' : '#ff8c00';

  const updateValue = useCallback(
    (clientY: number) => {
      if (!trackRef.current || disabled) return;
      const rect = trackRef.current.getBoundingClientRect();
      const y = Math.max(0, Math.min(height, clientY - rect.top));
      const newDb = positionToDb(y, height);
      onChange(Math.round(newDb * 10) / 10);
    },
    [onChange, disabled, height]
  );

  const handleMouseDown = (e: React.MouseEvent) => {
    if (disabled) return;
    e.preventDefault();
    setIsDragging(true);
    updateValue(e.clientY);
  };

  const handleWheel = (e: React.WheelEvent) => {
    if (disabled) return;
    e.preventDefault();
    const delta = e.deltaY > 0 ? -1 : 1;
    onChange(Math.max(MIN_DB, Math.min(MAX_DB, value + delta)));
  };

  useEffect(() => {
    if (!isDragging) return;
    const handleMouseMove = (e: MouseEvent) => updateValue(e.clientY);
    const handleMouseUp = () => setIsDragging(false);
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, updateValue]);

  return (
    <div
      ref={trackRef}
      className="relative cursor-pointer select-none"
      style={{ width: '16px', height: `${height}px` }}
      onMouseDown={handleMouseDown}
      onWheel={handleWheel}
      role="slider"
      aria-label="Lautstärke-Fader"
      aria-valuemin={MIN_DB}
      aria-valuemax={MAX_DB}
      aria-valuenow={value}
      aria-disabled={disabled}
    >
      {/* Track: 2px breit, zentriert */}
      <div
        className="absolute left-1/2 -translate-x-1/2 rounded-sm"
        style={{
          width: '2px',
          height: `${height - 6}px`,
          top: '3px',
          backgroundColor: 'rgba(255,255,255,0.03)',
        }}
      />
      {/* Fill */}
      <div
        className="absolute left-1/2 -translate-x-1/2 rounded-sm"
        style={{
          width: '2px',
          height: `${((value - MIN_DB) / (MAX_DB - MIN_DB)) * (height - 6)}px`,
          bottom: '3px',
          backgroundColor: fillColor,
          boxShadow: `0 0 4px ${fillColor}25`,
          opacity: disabled ? 0.3 : 1,
        }}
      />
      {/* Thumb: 14×9px mit Grip-Lines */}
      <div
        className="absolute left-1/2 -translate-x-1/2 rounded-sm flex flex-col items-center justify-center gap-[1px]"
        style={{
          width: '14px',
          height: '9px',
          top: `${thumbY}px`,
          marginTop: '-4.5px',
          background: 'linear-gradient(180deg, #484848, #282828)',
          border: '1px solid #555',
          boxShadow: '0 1px 2px rgba(0,0,0,0.5)',
          opacity: disabled ? 0.3 : 1,
        }}
      >
        <div style={{ width: '8px', height: '1px', background: 'rgba(255,255,255,0.15)' }} />
        <div style={{ width: '8px', height: '1px', background: 'rgba(255,255,255,0.15)' }} />
        <div style={{ width: '8px', height: '1px', background: 'rgba(255,255,255,0.15)' }} />
      </div>
    </div>
  );
}
