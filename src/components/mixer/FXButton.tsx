// Component: FXButton — FX-Chain Ein/Aus Toggle pro Strip

interface FXButtonProps {
  /** FX-Chain aktiv */
  active: boolean;
  /** Callback bei Klick */
  onClick: () => void;
}

/**
 * FX-Chain Toggle-Button
 * Cyan wenn aktiv, grau wenn inaktiv
 */
export default function FXButton({ active, onClick }: FXButtonProps) {
  return (
    <button
      className={`px-1.5 py-0.5 text-[5px] font-bold uppercase tracking-wider rounded-sm transition-all ${
        active ? 'bg-inox-cyan text-inox-bg' : 'bg-inox-subtle/30 text-inox-muted border border-inox-subtle'
      }`}
      onClick={onClick}
      aria-label="FX-Chain"
      aria-pressed={active}
    >
      FX
    </button>
  );
}
