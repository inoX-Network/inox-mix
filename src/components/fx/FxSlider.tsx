// Component: FxSlider — Horizontaler Parameter-Slider für FX-Module
// Spec: Track 5px Höhe, Thumb 6×8px, fill mit box-shadow
import { useState, useRef, useCallback, useEffect } from 'react';

interface FxSliderProps {
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
  color: 'cyan' | 'orange';
  disabled?: boolean;
}

/** Horizontaler Slider für FX-Parameter */
export default function FxSlider({ value, min, max, onChange, color, disabled = false }: FxSliderProps) {
  const [isDragging, setIsDragging] = useState(false);
  const trackRef = useRef<HTMLDivElement>(null);
  const fillColor = color === 'cyan' ? '#00e5ff' : '#ff8c00';

  const normalized = (value - min) / (max - min);

  const updateValue = useCallback(
    (clientX: number) => {
      if (!trackRef.current || disabled) return;
      const rect = trackRef.current.getBoundingClientRect();
      const w = rect.width;
      const x = Math.max(0, Math.min(w, clientX - rect.left));
      const newValue = min + (x / w) * (max - min);
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
    const handleMouseMove = (e: MouseEvent) => updateValue(e.clientX);
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
      className="relative w-full cursor-pointer select-none"
      style={{ height: '5px', background: 'rgba(255,255,255,0.03)', borderRadius: '3px' }}
      onMouseDown={handleMouseDown}
      role="slider"
      aria-valuemin={min}
      aria-valuemax={max}
      aria-valuenow={value}
      aria-disabled={disabled}
    >
      {/* Fill */}
      <div
        className="absolute left-0 top-0 h-full rounded-[3px]"
        style={{
          width: `${normalized * 100}%`,
          backgroundColor: fillColor,
          boxShadow: `0 0 4px ${fillColor}25`,
          opacity: disabled ? 0.3 : 1,
        }}
      />
      {/* Thumb: 6×8px */}
      <div
        className="absolute top-1/2 -translate-y-1/2 rounded-sm"
        style={{
          width: '6px',
          height: '8px',
          left: `${normalized * 100}%`,
          marginLeft: '-3px',
          background: 'linear-gradient(180deg, #555, #333)',
          border: '1px solid #666',
          boxShadow: '0 1px 2px rgba(0,0,0,0.4)',
          opacity: disabled ? 0.3 : 1,
        }}
      />
    </div>
  );
}
