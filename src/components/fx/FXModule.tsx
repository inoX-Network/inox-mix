// Komponente: FXModule — einzelnes Effekt-Modul in der FX-Chain

/** Einzelnes FX-Modul mit Parametern und Bypass */
interface FXModuleProps {
  /** Effekt-Name */
  name?: string;
  /** Effekt-Typ */
  type?: string;
  /** Effekt aktiv/bypass */
  enabled?: boolean;
  /** Callback für Bypass-Toggle */
  onToggle?: () => void;
}

function FXModule(_props: FXModuleProps) {
  // TODO: Modul-Header mit Name und Bypass-Toggle
  // TODO: Parameter-Slider je nach Effekt-Typ
  // TODO: Gain-Reduction Meter (bei Comp/Limiter)
  return (
    <div className="bg-inox-strip border border-[rgba(255,255,255,0.05)] rounded p-2">
      {/* TODO: FX-Modul Inhalt */}
    </div>
  );
}

export default FXModule;
