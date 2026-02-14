// Component: FxSlider — Horizontaler Parameter-Slider für FX-Module
import { useState, useRef, useCallback, useEffect } from 'react';

interface FxSliderProps {
  /** Aktueller Wert */
  value: number;
  /** Minimal-Wert */
  min: number;
  /** Maximal-Wert */
  max: number;
  /** Callback bei Änderung */
  onChange: (value: number) => void;
  /** Farbe (cyan oder orange) */
  color: 'cyan' | 'orange';
  /** Deaktiviert */
  disabled?: boolean;
}

const SLIDER_WIDTH = 100; // px

/**
 * Wert in Pixel-Position umrechnen
 */
function valueToPosition(value: number, min: number, max: number): number {
  const normalized = (value - min) / (max - min); // 0..1
  return normalized * SLIDER_WIDTH;
}

/**
 * Pixel-Position in Wert umrechnen
 */
function positionToValue(pos: number, min: number, max: number): number {
  const normalized = pos / SLIDER_WIDTH; // 0..1
  return min + normalized * (max - min);
}

/**
 * Horizontaler Slider für FX-Parameter
 */
export default function FxSlider({
  value,
  min,
  max,
  onChange,
  color,
  disabled = false,
}: FxSliderProps) {
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);

  const colorClass = color === 'cyan' ? 'bg-inox-cyan' : 'bg-inox-orange';
  const thumbX = valueToPosition(value, min, max);

  const updateValue = useCallback(
    (clientX: number) => {
      if (!trackRef.current || disabled) return;
      const rect = trackRef.current.getBoundingClientRect();
      const x = Math.max(0, Math.min(SLIDER_WIDTH, clientX - rect.left));
      const newValue = positionToValue(x, min, max);
      // Runde auf 1 Dezimalstelle
      onChange(Math.round(newValue * 10) / 10);
    },
    [onChange, disabled, min, max]
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
        aria-valuemin={min}
        aria-valuemax={max}
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
          className={`absolute top-1/2 -translate-y-1/2 w-2 h-2 rounded-full ${colorClass} ${
            disabled ? 'opacity-30' : 'opacity-100'
          } transition-opacity`}
          style={{
            left: `${thumbX}px`,
            marginLeft: '-4px',
          }}
        />
      </div>
    </div>
  );
}
