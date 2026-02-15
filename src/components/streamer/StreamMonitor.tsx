// Component: StreamMonitor — Stream Monitor mit 4 animierten Wellen
// Zeigt: Output Level, Duck Envelope, Bleeper, Voice FX

/**
 * Stream Monitor — Rechts neben Stream Master Fader
 *
 * 4 animierte SVG-Wellen übereinander:
 * 1. OUTPUT LEVEL — Orange, Sinuswelle, "-4.2 dB"
 * 2. DUCK ENVELOPE — Cyan, Rechteck-Muster (oben=normal, unten=geduckt), "-12 dB"
 * 3. BLEEPER — Rot, Flache Linie mit Spikes, "IDLE"
 * 4. VOICE FX — Cyan, High-Freq Muster, "Robot"
 */
export default function StreamMonitor() {
  return (
    <div className="flex-1 bg-black/[0.35] border border-inox-orange/[0.06] rounded-[5px] p-[5px] relative overflow-hidden min-h-0">
      {/* Header */}
      <div className="flex justify-between items-center mb-[3px]">
        <span className="text-[5px] font-bold tracking-[1.5px] text-inox-orange/[0.4] uppercase">
          STREAM MONITOR
        </span>
        <span className="text-[5px] text-white/[0.06]">
          B1 LIVE
        </span>
      </div>

      {/* Grid-Hintergrund (SVG Pattern, 15×15px) */}
      <svg
        className="absolute inset-0 w-full h-full pointer-events-none opacity-[0.02]"
        preserveAspectRatio="none"
      >
        <defs>
          <pattern id="sgrd" width="15" height="15" patternUnits="userSpaceOnUse">
            <path d="M 15 0 L 0 0 0 15" fill="none" stroke="white" strokeWidth="0.5" />
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#sgrd)" />
      </svg>

      {/* 4 Wellen-Reihen */}
      <div className="relative z-10 flex flex-col gap-[1px]">
        {/* Welle 1: OUTPUT LEVEL (Orange) */}
        <StreamWaveRow
          label="OUTPUT LEVEL"
          value="-4.2 dB"
          color="#ff8c00"
          gradId="sgO"
          speed={2.8}
          d1="M0,16 Q12,6 24,14 Q36,22 48,12 Q60,4 72,15 Q84,24 96,14 Q108,5 120,16 Q132,24 144,12 Q156,4 168,15 Q180,22 192,12 Q204,4 216,16 Q228,24 240,12 Q252,5 264,16 Q276,22 288,14 Q294,8 300,15"
          d2="M0,14 Q12,22 24,12 Q36,5 48,16 Q60,24 72,14 Q84,6 96,16 Q108,22 120,12 Q132,4 144,16 Q156,24 168,12 Q180,4 192,16 Q204,24 216,12 Q228,4 240,16 Q252,24 264,12 Q276,4 288,16 Q294,22 300,14"
        />

        {/* Welle 2: DUCK ENVELOPE (Cyan) */}
        <StreamWaveRow
          label="DUCK ENVELOPE"
          value="-12 dB"
          color="#00e5ff"
          gradId="sgC"
          speed={3.5}
          d1="M0,6 L30,6 L35,22 L80,22 L85,6 L150,6 L155,22 L210,22 L215,6 L280,6 L285,22 L300,22"
          d2="M0,22 L20,22 L25,6 L100,6 L105,22 L140,22 L145,6 L230,6 L235,22 L270,22 L275,6 L300,6"
        />

        {/* Welle 3: BLEEPER (Rot) */}
        <StreamWaveRow
          label="BLEEPER"
          value="IDLE"
          color="#ff1744"
          gradId="sgR"
          speed={5}
          d1="M0,24 L60,24 L62,24 L64,24 L100,24 L100,24 L160,24 L200,24 L240,24 L260,24 L300,24"
          d2="M0,24 L60,24 L62,6 L68,6 L70,24 L160,24 L162,6 L168,6 L170,24 L260,24 L262,6 L268,6 L270,24 L300,24"
        />

        {/* Welle 4: VOICE FX (Cyan) */}
        <StreamWaveRow
          label="VOICE FX"
          value="Robot"
          color="#00e5ff"
          gradId="sgP"
          speed={2}
          d1="M0,14 Q8,8 16,18 Q24,24 32,12 Q40,6 48,16 Q56,22 64,12 Q72,6 80,16 Q88,22 96,12 Q104,6 112,16 Q120,22 128,12 Q136,6 144,16 Q152,22 160,12 Q168,6 176,16 Q184,22 192,12 Q200,6 208,16 Q216,22 224,12 Q232,6 240,16 Q248,22 256,12 Q264,6 272,16 Q280,22 288,12 Q294,8 300,14"
          d2="M0,18 Q8,24 16,12 Q24,6 32,16 Q40,22 48,12 Q56,6 64,18 Q72,24 80,12 Q88,6 96,18 Q104,24 112,12 Q120,6 128,18 Q136,24 144,12 Q152,6 160,18 Q168,24 176,12 Q184,6 192,18 Q200,24 208,12 Q216,6 224,18 Q232,24 240,12 Q248,6 256,18 Q264,24 272,12 Q280,6 288,18 Q294,24 300,18"
        />
      </div>
    </div>
  );
}

/** Einzelne Wellen-Reihe mit Label, Wert und animierter SVG-Welle */
interface StreamWaveRowProps {
  label: string;
  value: string;
  color: string;
  gradId: string;
  speed: number;
  d1: string;
  d2: string;
}

function StreamWaveRow({ label, value, color, gradId, speed, d1, d2 }: StreamWaveRowProps) {
  return (
    <div className="relative h-[28px] border-b border-white/[0.015] last:border-b-0">
      {/* Label */}
      <span
        className="absolute top-[1px] left-[3px] text-[4.5px] font-bold tracking-[0.8px] z-10"
        style={{ color }}
      >
        {label}
      </span>

      {/* Wert */}
      <span
        className="absolute top-[1px] right-[3px] text-[5px] font-semibold z-10"
        style={{ color }}
      >
        {value}
      </span>

      {/* Animierte SVG-Welle */}
      <svg
        width="100%"
        height="100%"
        viewBox="0 0 300 28"
        preserveAspectRatio="none"
        className="absolute inset-0"
      >
        <defs>
          <linearGradient id={gradId} x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor={color} stopOpacity="0.25" />
            <stop offset="100%" stopColor={color} stopOpacity="0" />
          </linearGradient>
        </defs>

        {/* Stroke-Linie (animiert) */}
        <path
          d={d1}
          fill="none"
          stroke={color}
          strokeWidth="1.3"
          opacity="0.55"
        >
          <animate
            attributeName="d"
            dur={`${speed}s`}
            repeatCount="indefinite"
            values={`${d1};${d2};${d1}`}
          />
        </path>

        {/* Fill-Fläche (animiert) */}
        <path d={`${d1} L300,28 L0,28Z`} fill={`url(#${gradId})`} opacity="0.07">
          <animate
            attributeName="d"
            dur={`${speed}s`}
            repeatCount="indefinite"
            values={`${d1} L300,28 L0,28Z;${d2} L300,28 L0,28Z;${d1} L300,28 L0,28Z`}
          />
        </path>
      </svg>
    </div>
  );
}
