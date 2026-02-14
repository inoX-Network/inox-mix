// Komponente: Slider — horizontaler Regler (FX-Parameter)

/** Horizontaler Slider mit Label und Wert-Anzeige */
interface SliderProps {
  /** Aktueller Wert (0.0 - 1.0) */
  value?: number;
  /** Label links oben (4.5px) */
  label?: string;
  /** Farbe des Fill */
  color?: string;
  /** Callback bei Wertänderung */
  onChange?: (value: number) => void;
}

function Slider(_props: SliderProps) {
  // TODO: Track (5px hoch, rgba)
  // TODO: Farbiger Fill + box-shadow
  // TODO: Thumb (6x8px, Gradient)
  // TODO: Label links oben, Value rechts oben
  return (
    <div className="relative w-full h-5">
      {/* TODO: Slider-Inhalt */}
    </div>
  );
}

export default Slider;
