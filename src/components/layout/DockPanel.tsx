// Komponente: DockPanel — dockbares/andockbares Panel-System

/** Dockbares Panel das verschoben und angedockt werden kann */
interface DockPanelProps {
  /** Panel-Titel */
  title?: string;
  /** Panel-Inhalt */
  children?: React.ReactNode;
}

function DockPanel(_props: DockPanelProps) {
  // TODO: Drag-Handle (6 Dots oben-rechts)
  // TODO: Dock/Undock Logik
  // TODO: Resize-Handle
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px]">
      {/* TODO: Panel-Inhalt */}
    </div>
  );
}

export default DockPanel;
