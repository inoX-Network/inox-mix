// Komponente: WaveDisplay — einzelne animierte Wellenform (SVG)
import { useMemo } from 'react';

/** Animierte SVG-Wellenform mit Farbe und Gradient-Fill */
interface WaveDisplayProps {
  /** Wellenform-Farbe (hex) */
  color: string;
  /** Label der Wellenform (optional) */
  label?: string;
  /** Animations-Dauer in Sekunden */
  duration?: number;
  /** Wellen-Typ für unterschiedliche Shapes (0-3) */
  waveType?: number;
}

/**
 * Generiert SVG-Pfad für Wellenform
 * @param type Wellen-Typ (0-3 für verschiedene Shapes)
 * @param amplitude Amplitude der Welle
 */
function generateWavePath(type: number, amplitude: number): string {
  const width = 300;
  const height = 38;
  const midY = height / 2;
  const points = 30; // Anzahl der Punkte für die Welle

  let path = `M 0,${midY}`;

  for (let i = 1; i <= points; i++) {
    const x = (i / points) * width;
    const offset = type * 1.3; // Phasenverschiebung für verschiedene Wellen

    // Verschiedene Wellenformen basierend auf Typ
    let y: number;
    if (type === 0) {
      // Sanfte Sinuswelle
      y = midY + Math.sin((i / points) * Math.PI * 4 + offset) * amplitude;
    } else if (type === 1) {
      // Komplexere Welle mit Obertönen
      y = midY +
        (Math.sin((i / points) * Math.PI * 4 + offset) * amplitude * 0.7) +
        (Math.sin((i / points) * Math.PI * 8 + offset) * amplitude * 0.3);
    } else if (type === 2) {
      // Gezackte Welle
      y = midY +
        (Math.sin((i / points) * Math.PI * 3 + offset) * amplitude * 0.6) +
        (Math.sin((i / points) * Math.PI * 12 + offset) * amplitude * 0.4);
    } else {
      // Chaotische Welle mit vielen Frequenzen
      y = midY +
        (Math.sin((i / points) * Math.PI * 5 + offset) * amplitude * 0.5) +
        (Math.sin((i / points) * Math.PI * 11 + offset) * amplitude * 0.3) +
        (Math.sin((i / points) * Math.PI * 17 + offset) * amplitude * 0.2);
    }

    path += ` L ${x},${y}`;
  }

  path += ` L ${width},${midY}`;
  return path;
}

/**
 * Einzelne animierte Wellenform
 */
function WaveDisplay({
  color,
  label,
  duration = 3,
  waveType = 0,
}: WaveDisplayProps) {
  // Gradient-ID basierend auf Farbe (um Kollisionen zu vermeiden)
  const gradientId = useMemo(() => {
    return `wave-gradient-${color.replace('#', '')}-${Math.random().toString(36).substr(2, 9)}`;
  }, [color]);

  // Drei verschiedene Pfade für Animation
  const path1 = generateWavePath(waveType, 8);
  const path2 = generateWavePath(waveType, 12);
  const path3 = generateWavePath(waveType, 10);

  // Animation values (zwischen den 3 Pfaden wechseln)
  const animationValues = `${path1};${path2};${path3};${path1}`;

  return (
    <div className="relative w-full">
      {label && (
        <div className="text-[5px] font-bold uppercase text-gray-500 mb-1 tracking-wide">
          {label}
        </div>
      )}

      <svg
        viewBox="0 0 300 38"
        className="w-full"
        style={{ height: '38px' }}
        preserveAspectRatio="none"
      >
        {/* LinearGradient Definition (top→bottom, opacity 0.07) */}
        <defs>
          <linearGradient id={gradientId} x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stopColor={color} stopOpacity="0.07" />
            <stop offset="100%" stopColor={color} stopOpacity="0.02" />
          </linearGradient>
        </defs>

        {/* Gefüllte Wellenform (sehr subtil) */}
        <path
          d={path1}
          fill={`url(#${gradientId})`}
          stroke="none"
        >
          <animate
            attributeName="d"
            values={animationValues}
            dur={`${duration}s`}
            repeatCount="indefinite"
            calcMode="spline"
            keySplines="0.4 0 0.6 1; 0.4 0 0.6 1; 0.4 0 0.6 1"
            keyTimes="0; 0.33; 0.67; 1"
          />
        </path>

        {/* Stroke-Wellenform (Hauptlinie) */}
        <path
          d={path1}
          fill="none"
          stroke={color}
          strokeWidth="1.3"
          opacity="0.55"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <animate
            attributeName="d"
            values={animationValues}
            dur={`${duration}s`}
            repeatCount="indefinite"
            calcMode="spline"
            keySplines="0.4 0 0.6 1; 0.4 0 0.6 1; 0.4 0 0.6 1"
            keyTimes="0; 0.33; 0.67; 1"
          />
        </path>
      </svg>
    </div>
  );
}

export default WaveDisplay;
