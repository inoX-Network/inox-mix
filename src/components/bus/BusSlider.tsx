// Component: BusSlider — Horizontaler Volume-Slider für Output-Busse
import { useState, useRef, useCallback, useEffect } from 'react';

interface BusSliderProps {
  /** Aktueller Wert in dB (-50 bis +10) */
  value: number;
  /** Callback bei Wert-Änderung */
  onChange: (value: number) => void;
  /** Farbe (cyan für A-Busse, orange für B-Busse) */
  color: 'cyan' | 'orange';
  /** Deaktiviert (muted) */
  disabled?: boolean;
}

const MIN_DB = -50;
const MAX_DB = 10;
const SLIDER_WIDTH = 100; // px

/**
 * dB-Wert in Pixel-Position umrechnen (0 = links = -50dB, 100 = rechts = +10dB)
 */
function dbToPosition(db: number): number {
  const normalized = (db - MIN_DB) / (MAX_DB - MIN_DB); // 0..1
  return normalized * SLIDER_WIDTH;
}

/**
 * Pixel-Position in dB-Wert umrechnen
 */
function positionToDb(pos: number): number {
  const normalized = pos / SLIDER_WIDTH; // 0..1
  return MIN_DB + normalized * (MAX_DB - MIN_DB);
}

/**
 * Horizontaler Slider für Bus-Lautstärke
 */
export default function BusSlider({ value, onChange, color, disabled = false }: BusSliderProps) {
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);

  const colorClass = color === 'cyan' ? 'bg-inox-cyan' : 'bg-inox-orange';
  const thumbX = dbToPosition(value);

  const updateValue = useCallback(
    (clientX: number) => {
      if (!trackRef.current || disabled) return;
      const rect = trackRef.current.getBoundingClientRect();
      const x = Math.max(0, Math.min(SLIDER_WIDTH, clientX - rect.left));
      const newDb = positionToDb(x);
      onChange(Math.round(newDb * 10) / 10); // 1 Dezimalstelle
    },
    [onChange, disabled]
  );

  const handleMouseDown = (e: React.MouseEvent) => {
    if (disabled) return;
    e.preventDefault();
    setIsDragging(true);
    updateValue(e.clientX);
  };

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      updateValue(e.clientX);
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
    <div className="flex items-center">
      {/* Slider Track */}
      <div
        ref={trackRef}
        className="relative h-1 bg-inox-subtle rounded-full cursor-pointer select-none"
        style={{ width: `${SLIDER_WIDTH}px` }}
        onMouseDown={handleMouseDown}
        role="slider"
        aria-label="Bus-Lautstärke"
        aria-valuemin={MIN_DB}
        aria-valuemax={MAX_DB}
        aria-valuenow={value}
        aria-disabled={disabled}
      >
        {/* Fill */}
        <div
          className={`absolute left-0 top-0 h-full rounded-full ${colorClass} ${
            disabled ? 'opacity-30' : 'opacity-100'
          } transition-opacity`}
          style={{ width: `${thumbX}px` }}
        />
        {/* Thumb */}
        <div
          className="absolute top-1/2 -translate-y-1/2 rounded-[1px] transition-opacity"
          style={{
            width: '6px',
            height: '8px',
            left: `${thumbX}px`,
            marginLeft: '-3px',
            background: 'linear-gradient(180deg, #555 0%, #333 100%)',
            border: '1px solid #666',
            opacity: disabled ? 0.3 : 1,
          }}
        />
      </div>
    </div>
  );
}
