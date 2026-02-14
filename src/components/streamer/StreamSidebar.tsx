// Komponente: StreamSidebar — Slide-out Sidebar für Streamer-Features (270px rechts)

/** Streamer-Sidebar mit Master, Ducking, Bleeper, Voice FX und Soundboard */
interface StreamSidebarProps {
  /** Sidebar sichtbar */
  visible?: boolean;
  /** Callback zum Schließen */
  onClose?: () => void;
}

function StreamSidebar(_props: StreamSidebarProps) {
  // TODO: 270px breite Sidebar, rechts
  // TODO: "STREAMER" Header mit LIVE-Badge
  // TODO: Stream Master (Fader + Monitor)
  // TODO: Audio Protection (Ducking + Bleeper)
  // TODO: Voice FX Sektion
  // TODO: Soundboard Sektion
  return (
    <aside className="w-[270px] bg-inox-panel border-l border-[rgba(255,255,255,0.05)] flex flex-col overflow-y-auto">
      {/* TODO: Sidebar-Inhalt */}
    </aside>
  );
}

export default StreamSidebar;
