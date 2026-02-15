// Component: SidebarToggleTab — Vertikaler Toggle-Tab am rechten Bildschirmrand
// Spec: 24×90px, Gradient-BG, Arrow 8px, Label 6px/700/2px tracking, writing-mode vertical
import { useAppStore } from '../../stores/appStore';

/**
 * Vertikaler Tab am rechten Fensterrand zum Ein-/Ausblenden der Stream-Sidebar
 * Mockup: .sidebar-tab — 24×90px, gradient bg, border-radius 8px, writing-mode vertical-rl
 */
export default function SidebarToggleTab() {
  const sidebarOpen = useAppStore((s) => s.sidebarOpen);
  const toggleSidebar = useAppStore((s) => s.toggleSidebar);

  return (
    <button
      onClick={toggleSidebar}
      className="absolute right-0 top-1/2 -translate-y-1/2 z-40 flex flex-col items-center justify-center gap-[4px] transition-all"
      style={{
        width: '24px',
        height: '90px',
        background: 'linear-gradient(180deg, rgba(255,109,0,0.12), rgba(224,64,251,0.12))',
        border: '1px solid rgba(255,109,0,0.15)',
        borderRight: 'none',
        borderTopLeftRadius: '8px',
        borderBottomLeftRadius: '8px',
        writingMode: 'vertical-rl',
      }}
      aria-label={sidebarOpen ? 'Stream-Sidebar schließen' : 'Stream-Sidebar öffnen'}
    >
      {/* Pfeil-Indikator */}
      <span
        style={{
          fontSize: '8px',
          color: '#ff8c00',
          writingMode: 'horizontal-tb',
          transition: 'transform 0.3s',
          transform: sidebarOpen ? 'rotate(180deg)' : 'rotate(0deg)',
        }}
      >
        ◀
      </span>

      {/* Label "STREAM" vertikal */}
      <span
        style={{
          fontSize: '6px',
          fontWeight: 700,
          letterSpacing: '2px',
          color: '#ff8c00',
          textTransform: 'uppercase',
        }}
      >
        STREAM
      </span>
    </button>
  );
}
