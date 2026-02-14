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
interface TabBarProps {}

function TabBar(_props: TabBarProps) {
  const activeTab = useAppStore((s) => s.activeTab);
  const setActiveTab = useAppStore((s) => s.setActiveTab);

  return (
    <nav className="h-7 bg-inox-panel border-b border-[rgba(255,255,255,0.05)] flex items-center px-3 gap-1 shrink-0"
         role="tablist"
         aria-label="Hauptnavigation">
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

/** Einzelner Tab-Button */
interface TabButtonProps {
  /** Tab-ID */
  id: AppTab;
  /** Anzeige-Label */
  label: string;
  /** Ob der Tab aktiv ist */
  active: boolean;
  /** Klick-Handler */
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
        relative px-3 h-full flex items-center
        text-[6px] font-bold uppercase tracking-[1.5px]
        transition-colors
        ${active
          ? 'text-inox-cyan'
          : 'text-inox-faint hover:text-inox-dim'
        }
      `}
    >
      {label}
      {/* Aktiver Tab: Cyan Underline */}
      {active && (
        <div className="absolute bottom-0 left-1 right-1 h-[1.5px] bg-inox-cyan rounded-full shadow-[0_0_4px_rgba(0,229,255,0.4)]" />
      )}
    </button>
  );
}

export default TabBar;
