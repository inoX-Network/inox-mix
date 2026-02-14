// Component: Knob — Drehbarer Regler für Gain mit Drag-Interaktion
import { useState, useCallback, useEffect } from 'react';

interface KnobProps {
  /** Aktueller Wert in dB (-20 bis +20 für Gain) */
  value: number;
  /** Callback bei Wert-Änderung */
  onChange: (value: number) => void;
  /** Label (z.B. "GAIN") */
  label: string;
  /** Farbe (cyan für Hardware, orange für Virtual) */
  color: 'cyan' | 'orange';
  /** Min-Wert in dB */
  min?: number;
  /** Max-Wert in dB */
  max?: number;
  /** Größe in px */
  size?: number;
}

const DEFAULT_MIN = -20;
const DEFAULT_MAX = 20;
const DEFAULT_SIZE = 20;
const ROTATION_RANGE = 270; // Grad (von -135° bis +135°)

/**
 * dB-Wert in Rotation-Grad umrechnen (-135° bis +135°)
 */
function dbToRotation(db: number, min: number, max: number): number {
  const normalized = (db - min) / (max - min); // 0..1
  return -135 + normalized * ROTATION_RANGE;
}

/**
 * Rotary Knob für Gain-Steuerung
 */
export default function Knob({
  value,
  onChange,
  label,
  color,
  min = DEFAULT_MIN,
  max = DEFAULT_MAX,
  size = DEFAULT_SIZE,
}: KnobProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [startY, setStartY] = useState(0);
  const [startValue, setStartValue] = useState(0);

  const colorClass = color === 'cyan' ? 'bg-inox-cyan' : 'bg-inox-orange';
  const rotation = dbToRotation(value, min, max);

  const handleMouseDown = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      setIsDragging(true);
      setStartY(e.clientY);
      setStartValue(value);
    },
    [value]
  );

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      const deltaY = startY - e.clientY; // Hoch = positiv
      const sensitivity = 0.5; // dB pro Pixel
      const newValue = Math.max(min, Math.min(max, startValue + deltaY * sensitivity));
      onChange(Math.round(newValue * 10) / 10); // 1 Dezimalstelle
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
  }, [isDragging, startY, startValue, min, max, onChange]);

  return (
    <div className="flex flex-col items-center gap-0.5">
      {/* Knob */}
      <div
        className="relative rounded-full cursor-pointer select-none"
        style={{
          width: `${size}px`,
          height: `${size}px`,
          background: 'radial-gradient(circle, #1a1a1a 0%, #0d0d0d 100%)',
          border: '1.5px solid #2a2a2a',
        }}
        onMouseDown={handleMouseDown}
        role="slider"
        aria-label={label}
        aria-valuemin={min}
        aria-valuemax={max}
        aria-valuenow={value}
      >
        {/* Needle */}
        <div
          className={`absolute top-1/2 left-1/2 origin-bottom ${colorClass}`}
          style={{
            width: '1.5px',
            height: `${size / 2 - 2}px`,
            transform: `translate(-50%, -100%) rotate(${rotation}deg)`,
          }}
        />
        {/* Center Dot */}
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-1 h-1 bg-inox-panel rounded-full" />
      </div>
      {/* Label */}
      <span className={`text-[6px] font-bold tracking-wider ${color === 'cyan' ? 'text-inox-cyan' : 'text-inox-orange'}`}>
        {label}
      </span>
    </div>
  );
}
