// Root-Komponente: Header + TabBar + Content Area + Sidebar Slot

/** inoX-MIX Hauptanwendung — verwaltet Layout und Navigation */
interface AppProps {}

function App(_props: AppProps) {
  // TODO: Tab-State (Mixer, FX, Routing, Settings, Help)
  // TODO: Sidebar-State (Stream Sidebar ein/aus)
  return (
    <div className="min-h-screen bg-inox-bg font-oxanium text-[#cccccc]">
      {/* TODO: Header */}
      {/* TODO: TabBar */}
      {/* TODO: Content Area (je nach Tab) */}
      {/* TODO: Stream Sidebar (rechts, optional) */}
      <p className="p-4 text-sm opacity-50">inoX-MIX v0.3 — Skeleton</p>
    </div>
  );
}

export default App;
