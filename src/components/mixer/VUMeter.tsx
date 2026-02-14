// Komponente: VUMeter — vertikales VU-Meter mit 13 Segmenten

/** VU-Meter mit Peak + RMS Anzeige, farbige Segmente */
interface VUMeterProps {
  /** Peak-Pegel in dB */
  peak?: number;
  /** RMS-Pegel in dB */
  rms?: number;
  /** Kanal-Farbe (Segmente 0-8) */
  color?: string;
}

function VUMeter(_props: VUMeterProps) {
  // TODO: 13 Segmente vertikal
  // TODO: Segment 0-8: Kanal-Farbe
  // TODO: Segment 9-10: Amber (#e6a117)
  // TODO: Segment 11-12: Rot (#ff1744)
  // TODO: Segmentbreite 3.5px, border-radius 1px, gap 1px
  return (
    <div className="flex gap-[1px] h-full">
      {/* TODO: VU-Segmente */}
    </div>
  );
}

export default VUMeter;
