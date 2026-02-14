// Komponente: Fader — vertikaler Lautstärke-Regler

/** Vertikaler Fader mit Track, Fill und Thumb */
interface FaderProps {
  /** Aktueller Wert in dB (-50 bis +10) */
  value?: number;
  /** Farbe des Fill (Cyan oder Orange) */
  color?: string;
  /** Callback bei Wertänderung */
  onChange?: (value: number) => void;
}

function Fader(_props: FaderProps) {
  // TODO: Vertikaler Track (2px breit)
  // TODO: Farbiger Fill mit box-shadow
  // TODO: Thumb (14x9px, Gradient)
  // TODO: Drag-Interaktion
  return (
    <div className="relative w-3 h-full flex items-center justify-center">
      {/* TODO: Fader-Inhalt */}
    </div>
  );
}

export default Fader;
