// Komponente: BusButton — Bus-Routing Toggle (A1, A2, B1, B2)

/** Bus-Zuweisungs-Button als Chip/Tag */
interface BusButtonProps {
  /** Bus-Name (z.B. "A1", "B1") */
  busId?: string;
  /** Aktiv/Inaktiv */
  active?: boolean;
  /** Farbe (Cyan für A-Busse, Orange für B-Busse) */
  color?: string;
  /** Callback bei Klick */
  onClick?: () => void;
}

function BusButton(_props: BusButtonProps) {
  // TODO: Chip-Style (1.5px 4px Padding)
  // TODO: Aktiv: gefüllter Hintergrund
  // TODO: Inaktiv: nur Border
  return (
    <button className="px-1 py-[1.5px] text-[5px] font-bold uppercase tracking-[0.4px] border rounded-sm">
      {/* TODO: Bus-Label */}
    </button>
  );
}

export default BusButton;
