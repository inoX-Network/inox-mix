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
}

const MIN_DB = -50;
const MAX_DB = 10;
const FADER_HEIGHT = 90; // px
const THUMB_HEIGHT = 9; // px

/**
 * dB-Wert in Pixel-Position umrechnen (0 = oben = +10dB, 90 = unten = -50dB)
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
 * Vertikaler Fader für Lautstärke
 */
export default function Fader({ value, onChange, color, disabled = false }: FaderProps) {
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);

  const colorClass = color === 'cyan' ? 'bg-inox-cyan' : 'bg-inox-orange';
  const thumbY = dbToPosition(value);

  const updateValue = useCallback(
    (clientY: number) => {
      if (!trackRef.current || disabled) return;
      const rect = trackRef.current.getBoundingClientRect();
      const y = Math.max(0, Math.min(FADER_HEIGHT, clientY - rect.top));
      const newDb = positionToDb(y);
      onChange(Math.round(newDb * 10) / 10); // 1 Dezimalstelle
    },
    [onChange, disabled]
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
    const delta = e.deltaY > 0 ? -1 : 1; // Runter = leiser, Hoch = lauter
    onChange(Math.max(MIN_DB, Math.min(MAX_DB, value + delta)));
  };

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      updateValue(e.clientY);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, updateValue]);

  return (
    <div className="flex items-center gap-1">
      {/* Fader Track */}
      <div
        ref={trackRef}
        className="relative w-1 bg-inox-subtle rounded-full cursor-pointer select-none"
        style={{ height: `${FADER_HEIGHT}px` }}
        onMouseDown={handleMouseDown}
        onWheel={handleWheel}
        role="slider"
        aria-label="Lautstärke-Fader"
        aria-valuemin={MIN_DB}
        aria-valuemax={MAX_DB}
        aria-valuenow={value}
        aria-disabled={disabled}
      >
        {/* Fader Thumb */}
        <div
          className={`absolute left-1/2 -translate-x-1/2 w-3.5 h-2 rounded-sm ${colorClass} ${
            disabled ? 'opacity-30' : 'opacity-100'
          } transition-opacity`}
          style={{
            top: `${thumbY}px`,
            marginTop: `-${THUMB_HEIGHT / 2}px`,
          }}
        />
      </div>
    </div>
  );
}
