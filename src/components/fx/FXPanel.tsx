// Komponente: FXPanel — Detail-Panel für die FX-Chain eines Kanals

/** FX-Chain Detail-Ansicht mit allen Effekt-Modulen */
interface FXPanelProps {
  /** Kanal-ID dessen FX-Chain angezeigt wird */
  channelId?: string;
}

function FXPanel(_props: FXPanelProps) {
  // TODO: Kanal-Name Header
  // TODO: FX-Module in Reihenfolge: HPF → Denoise → Gate → DeEsser → EQ → Comp → Limiter → AutoGain
  // TODO: Drag-Reorder der Module
  // TODO: Bypass-Toggle pro Modul
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-3">
      {/* TODO: FX-Chain Module */}
    </div>
  );
}

export default FXPanel;
