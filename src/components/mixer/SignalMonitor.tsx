// Component: SignalMonitor — Zentraler Analyzer mit 4 animierten Wellen
// Spec: Grid 20×20, stroke 0.5, wave row 38px, viewBox 0 0 300 38

/** Signal Monitor — Großer zentraler Analyzer */
export default function SignalMonitor() {
  return (
    <div className="flex-1 bg-black/[0.3] border border-white/[0.03] rounded-md p-[6px] relative overflow-hidden min-h-0">
      {/* Header: 6px title, 5px subtitle */}
      <div className="flex justify-between items-center mb-1">
        <span
          style={{
            fontSize: '6px',
            fontWeight: 700,
            letterSpacing: '2px',
            color: 'rgba(255,255,255,0.12)',
            textTransform: 'uppercase',
          }}
        >
          SIGNAL MONITOR
        </span>
        <span style={{ fontSize: '5px', color: 'rgba(255,255,255,0.06)' }}>
          LIVE &bull; 48kHz
        </span>
      </div>

      {/* Grid-Hintergrund: SVG Pattern 20×20px, stroke 0.5 */}
      <svg
        className="absolute inset-0 w-full h-full pointer-events-none"
        style={{ opacity: 0.025 }}
        preserveAspectRatio="none"
      >
        <defs>
          <pattern id="grid" width="20" height="20" patternUnits="userSpaceOnUse">
            <path d="M 20 0 L 0 0 0 20" fill="none" stroke="white" strokeWidth="0.5" />
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#grid)" />
      </svg>

      {/* 4 Wellen-Reihen */}
      <div className="relative z-10 flex flex-col gap-[1px]">
        {/* Welle 1: MASTER LEVEL (Cyan) */}
        <WaveRow
          label="MASTER LEVEL" value="-2.4 dB" color="#00e5ff" opacity={1}
          gradId="gC" speed={3}
          d1="M0,22 Q15,8 30,18 Q45,28 60,15 Q75,5 90,20 Q105,30 120,16 Q135,6 150,19 Q165,28 180,14 Q195,4 210,18 Q225,30 240,16 Q255,8 270,20 Q285,28 300,19"
          d2="M0,18 Q15,28 30,14 Q45,6 60,22 Q75,30 90,16 Q105,8 120,20 Q135,28 150,15 Q165,5 180,22 Q195,30 210,14 Q225,6 240,20 Q255,28 270,16 Q285,8 300,22"
        />
        {/* Welle 2: NOISE GATE (Cyan 60%) */}
        <WaveRow
          label="NOISE GATE" value="OPEN" color="#00e5ff" opacity={0.6}
          gradId="gG" speed={4}
          d1="M0,30 L20,30 L22,8 L80,8 L82,30 L120,30 L122,10 L200,10 L202,30 L250,30 L252,8 L290,8 L292,30 L300,30"
          d2="M0,30 L40,30 L42,10 L140,10 L142,30 L170,30 L172,8 L260,8 L262,30 L300,30 L300,30 L300,30 L300,30 L300,30"
        />
        {/* Welle 3: COMPRESSOR GR (Orange) */}
        <WaveRow
          label="COMPRESSOR GR" value="-6.2 dB" color="#ff8c00" opacity={1}
          gradId="gY" speed={3.5}
          d1="M0,30 Q20,28 40,24 Q60,16 80,20 Q100,28 120,30 Q140,32 160,22 Q180,12 200,18 Q220,26 240,30 Q260,34 280,28 Q290,24 300,26"
          d2="M0,26 Q20,22 40,28 Q60,32 80,24 Q100,14 120,18 Q140,26 160,30 Q180,34 200,26 Q220,18 240,22 Q260,28 280,32 Q290,30 300,28"
        />
        {/* Welle 4: AI DENOISE (Orange 60%) */}
        <WaveRow
          label="AI DENOISE" value="-18 dB" color="#ff8c00" opacity={0.6}
          gradId="gP" speed={2.5}
          d1="M0,20 Q10,14 20,22 Q30,28 40,18 Q50,12 60,20 Q70,26 80,16 Q90,10 100,20 Q110,28 120,16 Q130,10 140,22 Q150,28 160,16 Q170,10 180,20 Q190,28 200,16 Q210,10 220,22 Q230,28 240,16 Q250,10 260,22 Q270,28 280,16 Q290,12 300,20"
          d2="M0,22 Q10,28 20,16 Q30,10 40,22 Q50,28 60,16 Q70,10 80,22 Q90,28 100,16 Q110,10 120,22 Q130,28 140,16 Q150,10 160,22 Q170,28 180,16 Q190,10 200,22 Q210,28 220,16 Q230,10 240,22 Q250,28 260,16 Q270,10 280,22 Q290,28 300,18"
        />
      </div>
    </div>
  );
}

/** Einzelne Wellen-Reihe — Spec: h=38px, viewBox 300×38, stroke-width 1.3, opacity 0.55 */
interface WaveRowProps {
  label: string; value: string; color: string; opacity: number;
  gradId: string; speed: number; d1: string; d2: string;
}

function WaveRow({ label, value, color, opacity, gradId, speed, d1, d2 }: WaveRowProps) {
  return (
    <div className="relative border-b border-white/[0.015] last:border-b-0" style={{ height: '38px' }}>
      {/* Label: 5px */}
      <span
        className="absolute top-[2px] left-[5px] z-10"
        style={{ fontSize: '5px', fontWeight: 700, letterSpacing: '1px', color, opacity }}
      >
        {label}
      </span>
      {/* Wert: 5.5px */}
      <span
        className="absolute top-[2px] right-[5px] z-10"
        style={{ fontSize: '5.5px', fontWeight: 600, color, opacity }}
      >
        {value}
      </span>
      {/* SVG-Welle */}
      <svg width="100%" height="100%" viewBox="0 0 300 38" preserveAspectRatio="none" className="absolute inset-0">
        <defs>
          <linearGradient id={gradId} x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor={color} stopOpacity="0.25" />
            <stop offset="100%" stopColor={color} stopOpacity="0" />
          </linearGradient>
        </defs>
        <path d={d1} fill="none" stroke={color} strokeWidth="1.3" opacity={opacity * 0.55}>
          <animate attributeName="d" dur={`${speed}s`} repeatCount="indefinite" values={`${d1};${d2};${d1}`} />
        </path>
        <path d={`${d1} L300,38 L0,38Z`} fill={`url(#${gradId})`} opacity="0.07">
          <animate attributeName="d" dur={`${speed}s`} repeatCount="indefinite" values={`${d1} L300,38 L0,38Z;${d2} L300,38 L0,38Z;${d1} L300,38 L0,38Z`} />
        </path>
      </svg>
    </div>
  );
}
