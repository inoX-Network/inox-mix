// Komponente: Slider — horizontaler Regler (FX-Parameter)
import { useState, useRef, useCallback, useEffect } from 'react';

/** Horizontaler Slider mit Label und Wert-Anzeige */
interface SliderProps {
  /** Aktueller Wert (0.0 - 1.0) */
  value: number;
  /** Label links oben (4.5px) */
  label: string;
  /** Farbe des Fill (hex) */
  color?: string;
  /** Callback bei Wertänderung */
  onChange: (value: number) => void;
  /** Einheit für Wert-Anzeige (z.B. "%", "dB", "Hz") */
  unit?: string;
  /** Deaktiviert */
  disabled?: boolean;
}

const TRACK_HEIGHT = 5; // px

/**
 * Horizontaler Slider für FX-Parameter
 */
function Slider({
  value,
  label,
  color = '#00e5ff',
  onChange,
  unit = '%',
  disabled = false
}: SliderProps) {
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);

  const displayValue = unit === '%'
    ? Math.round(value * 100)
    : Math.round(value * 10) / 10;

  const updateValue = useCallback(
    (clientX: number) => {
      if (!trackRef.current || disabled) return;
      const rect = trackRef.current.getBoundingClientRect();
      const x = Math.max(0, Math.min(rect.width, clientX - rect.left));
      const newValue = x / rect.width;
      onChange(Math.round(newValue * 100) / 100); // 2 Dezimalstellen
    },
    [onChange, disabled]
  );

  const handleMouseDown = (e: React.MouseEvent) => {
    if (disabled) return;
    e.preventDefault();
    setIsDragging(true);
    updateValue(e.clientX);
  };

  const handleWheel = (e: React.WheelEvent) => {
    if (disabled) return;
    e.preventDefault();
    const delta = e.deltaY > 0 ? -0.05 : 0.05; // Runter = weniger, Hoch = mehr
    onChange(Math.max(0, Math.min(1, value + delta)));
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
    <div className="relative w-full">
      {/* Labels */}
      <div className="flex items-center justify-between mb-1">
        <span
          className="text-[4.5px] font-bold uppercase tracking-wide text-gray-400"
          style={{ letterSpacing: '0.4px' }}
        >
          {label}
        </span>
        <span
          className="text-[5px] font-semibold tabular-nums"
          style={{ color }}
        >
          {displayValue}{unit}
        </span>
      </div>

      {/* Slider Track */}
      <div
        ref={trackRef}
        className="relative w-full rounded-[3px] cursor-pointer select-none"
        style={{
          height: `${TRACK_HEIGHT}px`,
          backgroundColor: 'rgba(255, 255, 255, 0.03)',
        }}
        onMouseDown={handleMouseDown}
        onWheel={handleWheel}
        role="slider"
        aria-label={label}
        aria-valuemin={0}
        aria-valuemax={1}
        aria-valuenow={value}
        aria-disabled={disabled}
      >
        {/* Fill */}
        <div
          className="absolute left-0 top-0 h-full rounded-[3px] transition-opacity"
          style={{
            width: `${value * 100}%`,
            backgroundColor: color,
            boxShadow: `0 0 4px ${color}25`,
            opacity: disabled ? 0.3 : 1,
          }}
        />

        {/* Thumb */}
        <div
          className="absolute top-1/2 -translate-y-1/2 w-[6px] h-[8px] rounded-[1px] transition-opacity"
          style={{
            left: `calc(${value * 100}% - 3px)`, // Zentriert auf Position
            background: 'linear-gradient(180deg, #555 0%, #333 100%)',
            border: '1px solid #666',
            opacity: disabled ? 0.3 : 1,
          }}
        />
      </div>
    </div>
  );
}

export default Slider;
