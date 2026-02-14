// Komponente: FXButton — FX-Chain Ein/Aus Toggle pro Strip

/** FX-Chain Toggle-Button auf dem Channel Strip */
interface FXButtonProps {
  /** FX-Chain aktiv */
  active?: boolean;
  /** Callback bei Klick */
  onClick?: () => void;
}

function FXButton(_props: FXButtonProps) {
  // TODO: "FX" Label
  // TODO: Aktiv: Cyan-Hintergrund
  // TODO: Inaktiv: Grau, gedimmt
  return (
    <button className="text-[5px] font-bold uppercase tracking-wider">
      {/* TODO: FX-Label */}
    </button>
  );
}

export default FXButton;
