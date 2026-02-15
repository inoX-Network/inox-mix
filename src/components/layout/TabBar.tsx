// Komponente: TabBar — Tab-Navigation (Mixer, FX, Routing, Apps, Settings, Help)

import { useAppStore, APP_TABS } from '../../stores/appStore';
import type { AppTab } from '../../stores/appStore';

/** Button-ID Zuordnung für Tab-Buttons (fortlaufend ab btn-002) */
const TAB_BUTTON_IDS: Record<AppTab, string> = {
  mixer: 'btn-002-a',
  fx: 'btn-002-b',
  routing: 'btn-002-c',
  apps: 'btn-002-d',
  settings: 'btn-002-e',
  help: 'btn-002-f',
};

/** Tab-Leiste für Haupt-Navigation zwischen den Ansichten */
function TabBar() {
  const activeTab = useAppStore((s) => s.activeTab);
  const setActiveTab = useAppStore((s) => s.setActiveTab);

  return (
    <nav
      className="flex items-center gap-[1px] px-3 py-[1px] shrink-0"
      style={{ background: 'rgba(255,255,255,0.02)', borderRadius: '3px' }}
      role="tablist"
      aria-label="Hauptnavigation"
    >
      {APP_TABS.map((tab) => (
        <TabButton
          key={tab.id}
          id={tab.id}
          label={tab.label}
          active={activeTab === tab.id}
          onClick={() => setActiveTab(tab.id)}
        />
      ))}
    </nav>
  );
}

/** Einzelner Tab-Button (Pill-Style wie Mockup) */
interface TabButtonProps {
  id: AppTab;
  label: string;
  active: boolean;
  onClick: () => void;
}

function TabButton({ id, label, active, onClick }: TabButtonProps) {
  return (
    <button
      id={TAB_BUTTON_IDS[id]}
      role="tab"
      aria-selected={active}
      aria-label={`Tab: ${label}`}
      onClick={onClick}
      className={`
        px-[9px] py-[3px] rounded-sm border-none
        text-[7px] font-bold uppercase tracking-[1.3px]
        transition-colors
        ${active
          ? 'bg-inox-cyan text-black'
          : 'bg-transparent text-white/[0.22] hover:text-white/[0.35]'
        }
      `}
    >
      {label}
    </button>
  );
}

export default TabBar;
