// Komponente: Knob — drehbarer Regler (Gain, Pan, etc.)

/** Drehregler mit Needle und Label */
interface KnobProps {
  /** Aktueller Wert (0.0 - 1.0) */
  value?: number;
  /** Label unter dem Knob */
  label?: string;
  /** Größe in px (16, 18, 20, 22, 24) */
  size?: number;
  /** Needle-Farbe */
  color?: string;
  /** Callback bei Wertänderung */
  onChange?: (value: number) => void;
}

function Knob(_props: KnobProps) {
  // TODO: Kreisförmiger Knob (radial-gradient)
  // TODO: Border (1.5px solid #2a2a2a)
  // TODO: Needle (1.5px, Farbe je nach Kontext)
  // TODO: Drag-Rotation
  return (
    <div className="flex flex-col items-center gap-0.5">
      {/* TODO: Knob-Inhalt */}
    </div>
  );
}

export default Knob;
