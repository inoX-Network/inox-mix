// Component: Knob — Drehbarer Regler für Gain mit Drag-Interaktion
import { useState, useCallback, useEffect } from 'react';

interface KnobProps {
  /** Aktueller Wert in dB */
  value: number;
  /** Callback bei Wert-Änderung */
  onChange: (value: number) => void;
  /** Label (z.B. "GAIN") */
  label: string;
  /** Farbe (cyan für Hardware, orange für Virtual) */
  color: 'cyan' | 'orange';
  /** Min-Wert */
  min?: number;
  /** Max-Wert */
  max?: number;
  /** Größe in px (default: 20) */
  size?: number;
}

const ROTATION_RANGE = 270;

/** dB-Wert in Rotation-Grad umrechnen (-135° bis +135°) */
function dbToRotation(db: number, min: number, max: number): number {
  const normalized = (db - min) / (max - min);
  return -135 + normalized * ROTATION_RANGE;
}

/** Rotary Knob — Spec: radial-gradient, 1.5px border, 1.5px needle */
export default function Knob({
  value,
  onChange,
  label,
  color,
  min = -20,
  max = 20,
  size = 20,
}: KnobProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [startY, setStartY] = useState(0);
  const [startValue, setStartValue] = useState(0);

  const rotation = dbToRotation(value, min, max);
  const needleHeight = Math.round(size * 0.28);
  const fillColor = color === 'cyan' ? '#00e5ff' : '#ff8c00';

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
      const deltaY = startY - e.clientY;
      const sensitivity = 0.5;
      const newValue = Math.max(min, Math.min(max, startValue + deltaY * sensitivity));
      onChange(Math.round(newValue * 10) / 10);
    };
    const handleMouseUp = () => setIsDragging(false);
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, startY, startValue, min, max, onChange]);

  return (
    <div className="inline-flex flex-col items-center gap-[1px]">
      {/* Label über Knob */}
      <span
        className="text-[5px] font-bold tracking-[0.5px] uppercase"
        style={{ color: 'rgba(255,255,255,0.2)' }}
      >
        {label}
      </span>
      {/* Knob */}
      <div
        className="relative rounded-full cursor-ns-resize select-none"
        style={{
          width: `${size}px`,
          height: `${size}px`,
          background: 'radial-gradient(circle at 40% 35%, #353535, #181818)',
          border: '1.5px solid #2a2a2a',
          boxShadow: '0 1px 3px rgba(0,0,0,0.4)',
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
          className="absolute top-1/2 left-1/2 origin-bottom rounded-sm"
          style={{
            width: '1.5px',
            height: `${needleHeight}px`,
            backgroundColor: fillColor,
            boxShadow: `0 0 3px ${fillColor}40`,
            transform: `translate(-50%, -100%) rotate(${rotation}deg)`,
          }}
        />
      </div>
    </div>
  );
}
