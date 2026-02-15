// Component: VUMeter — 13-Segment vertikales VU-Meter mit Peak + RMS

interface VUMeterProps {
  /** Peak-Pegel in dB (-60 bis 0) */
  peak: number;
  /** RMS-Pegel in dB (-60 bis 0) */
  rms: number;
  /** Kanal-Farbe (cyan für Hardware, orange für Virtual) */
  color: 'cyan' | 'orange';
  /** Höhe in px (optional, default 70) */
  height?: number;
}

const SEGMENTS = 13;
const MIN_DB = -60;
const MAX_DB = 0;
const SEGMENT_DB_RANGE = (MAX_DB - MIN_DB) / SEGMENTS;

/** dB-Wert in Segment-Index umrechnen (0 = unten = -60dB, 12 = oben = 0dB) */
function dbToSegment(db: number): number {
  if (db <= MIN_DB) return -1;
  if (db >= MAX_DB) return SEGMENTS - 1;
  return Math.floor((db - MIN_DB) / SEGMENT_DB_RANGE);
}

/** Segment-Farbe: 0-8 = Kanalfarbe, 9-10 = Amber, 11-12 = Rot */
function getSegmentColor(index: number, color: 'cyan' | 'orange'): string {
  if (index >= 11) return '#ff1744';
  if (index >= 9) return '#e6a117';
  return color === 'cyan' ? '#00e5ff' : '#ff8c00';
}

/** 13-Segment VU-Meter (vertikal) — Spec: 3.5px breit, 1px gap, 1px radius */
export default function VUMeter({ peak, rms, color, height = 70 }: VUMeterProps) {
  const peakSegment = dbToSegment(peak);
  const rmsSegment = dbToSegment(rms);
  const segH = Math.floor(height / SEGMENTS - 1);

  return (
    <div className="flex flex-col-reverse gap-[1px]" style={{ height: `${height}px` }}>
      {Array.from({ length: SEGMENTS }).map((_, index) => {
        const isPeakActive = index <= peakSegment;
        const isRmsActive = index <= rmsSegment;
        const segmentColor = getSegmentColor(index, color);

        return (
          <div
            key={index}
            style={{
              width: '3.5px',
              height: `${segH}px`,
              borderRadius: '1px',
              backgroundColor: isPeakActive ? segmentColor : 'rgba(255,255,255,0.02)',
              opacity: isPeakActive ? (isRmsActive ? 1 : 0.4) : 1,
            }}
          />
        );
      })}
    </div>
  );
}
