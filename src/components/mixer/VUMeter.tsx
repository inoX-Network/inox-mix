// Component: VUMeter — 13-Segment vertikales VU-Meter mit Peak + RMS

interface VUMeterProps {
  /** Peak-Pegel in dB (-60 bis 0) */
  peak: number;
  /** RMS-Pegel in dB (-60 bis 0) */
  rms: number;
  /** Kanal-Farbe (cyan für Hardware, orange für Virtual) */
  color: 'cyan' | 'orange';
}

const SEGMENTS = 13;
const MIN_DB = -60;
const MAX_DB = 0;
const SEGMENT_DB_RANGE = (MAX_DB - MIN_DB) / SEGMENTS; // ~4.6 dB pro Segment

/**
 * dB-Wert in Segment-Index umrechnen (0 = unten = -60dB, 12 = oben = 0dB)
 */
function dbToSegment(db: number): number {
  if (db <= MIN_DB) return -1; // Kein Segment aktiv
  if (db >= MAX_DB) return SEGMENTS - 1;
  return Math.floor((db - MIN_DB) / SEGMENT_DB_RANGE);
}

/**
 * Segment-Farbe ermitteln
 * Segment 0-8: Kanal-Farbe
 * Segment 9-10: Amber (#e6a117)
 * Segment 11-12: Rot (#ff1744)
 */
function getSegmentColor(index: number, color: 'cyan' | 'orange'): string {
  if (index >= 11) return '#ff1744'; // Rot (Clip-Bereich)
  if (index >= 9) return '#e6a117'; // Amber (Warnung)
  return color === 'cyan' ? '#00e5ff' : '#ff8c00'; // Kanal-Farbe
}

/**
 * 13-Segment VU-Meter (vertikal, grün→amber→rot)
 */
export default function VUMeter({ peak, rms, color }: VUMeterProps) {
  const peakSegment = dbToSegment(peak);
  const rmsSegment = dbToSegment(rms);

  return (
    <div className="flex flex-col-reverse gap-[1px] h-full">
      {Array.from({ length: SEGMENTS }).map((_, index) => {
        const isPeakActive = index <= peakSegment;
        const isRmsActive = index <= rmsSegment;
        const segmentColor = getSegmentColor(index, color);

        return (
          <div
            key={index}
            className="w-[3.5px] h-full rounded-[1px] transition-opacity"
            style={{
              backgroundColor: segmentColor,
              opacity: isPeakActive ? (isRmsActive ? 1 : 0.4) : 0.1, // RMS voll, Peak gedimmt, Inaktiv sehr dunkel
            }}
          />
        );
      })}
    </div>
  );
}
