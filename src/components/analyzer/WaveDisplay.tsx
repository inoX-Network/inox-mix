// Komponente: WaveDisplay — einzelne animierte Wellenform (SVG)

/** Animierte SVG-Wellenform mit Farbe und Gradient-Fill */
interface WaveDisplayProps {
  /** Wellenform-Farbe */
  color?: string;
  /** Label der Wellenform */
  label?: string;
}

function WaveDisplay(_props: WaveDisplayProps) {
  // TODO: SVG Path mit animiertem d-Attribut
  // TODO: LinearGradient Fill (top→bottom, opacity 0.07)
  // TODO: Stroke-Width 1.3, Opacity 0.55
  return (
    <svg viewBox="0 0 300 38" className="w-full">
      {/* TODO: Animierte Wellenform */}
    </svg>
  );
}

export default WaveDisplay;
