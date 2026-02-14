// Komponente: TabBar — Tab-Navigation (Mixer, FX, Routing, Settings, Help)

/** Tab-Leiste für Haupt-Navigation zwischen den Ansichten */
interface TabBarProps {
  /** Aktuell aktiver Tab */
  activeTab?: string;
  /** Callback bei Tab-Wechsel */
  onTabChange?: (tab: string) => void;
}

function TabBar(_props: TabBarProps) {
  // TODO: Tabs: Mixer, FX Chain, Routing, Apps, Settings, Help
  // TODO: Aktiver Tab hervorheben (Cyan Underline)
  return (
    <nav className="h-7 bg-inox-panel border-b border-[rgba(255,255,255,0.05)] flex items-center px-3 gap-4">
      {/* TODO: Tab-Buttons */}
    </nav>
  );
}

export default TabBar;
