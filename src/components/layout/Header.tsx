// Komponente: Header — obere Leiste mit Logo, PipeWire-Status und System-Info

import { useAppStore } from '../../stores/appStore';

/** Header-Leiste mit App-Name, PipeWire-Status und Audio-Parametern */
interface HeaderProps {}

function Header(_props: HeaderProps) {
  const systemInfo = useAppStore((s) => s.systemInfo);
  const pipewireWarning = useAppStore((s) => s.pipewireWarning);
  const sidebarOpen = useAppStore((s) => s.sidebarOpen);
  const toggleSidebar = useAppStore((s) => s.toggleSidebar);

  /** PipeWire-Status: verbunden oder Warnung */
  const pwConnected = systemInfo?.pipewire_running && !pipewireWarning;

  return (
    <header className="h-8 bg-inox-panel border-b border-[rgba(255,255,255,0.05)] flex items-center px-3 justify-between shrink-0">
      {/* Links: Logo + Version */}
      <div className="flex items-center gap-2">
        <span className="text-[11px] font-extrabold tracking-[2px] text-inox-cyan uppercase">
          inoX-MIX
        </span>
        <span className="text-[9px] font-medium text-inox-muted tracking-wider">
          v{systemInfo?.app_version ?? '0.3'}
        </span>
      </div>

      {/* Mitte: PipeWire-Status + Audio-Parameter */}
      <div className="flex items-center gap-4">
        {/* PipeWire-Status-Dot */}
        <div className="flex items-center gap-1.5">
          <div
            className={`w-[6px] h-[6px] rounded-full ${
              pwConnected
                ? 'bg-inox-green shadow-[0_0_4px_rgba(76,175,80,0.5)]'
                : 'bg-inox-red shadow-[0_0_4px_rgba(255,23,68,0.5)]'
            }`}
          />
          <span className="text-[5px] font-bold uppercase tracking-[0.5px] text-inox-dim">
            {pwConnected ? 'PW OK' : 'PW OFFLINE'}
          </span>
        </div>

        {/* Audio-Parameter */}
        {systemInfo && (
          <div className="flex items-center gap-3">
            <div className="flex flex-col items-center">
              <span className="text-[5px] font-bold uppercase tracking-[0.5px] text-inox-faint">
                RATE
              </span>
              <span className="text-[8px] font-semibold text-inox-dim">
                {(systemInfo.sample_rate / 1000).toFixed(1)}k
              </span>
            </div>
            <div className="flex flex-col items-center">
              <span className="text-[5px] font-bold uppercase tracking-[0.5px] text-inox-faint">
                BUFFER
              </span>
              <span className="text-[8px] font-semibold text-inox-dim">
                {systemInfo.buffer_size}
              </span>
            </div>
            <div className="flex flex-col items-center">
              <span className="text-[5px] font-bold uppercase tracking-[0.5px] text-inox-faint">
                LATENZ
              </span>
              <span className="text-[8px] font-semibold text-inox-dim">
                {((systemInfo.buffer_size / systemInfo.sample_rate) * 1000).toFixed(1)}ms
              </span>
            </div>
          </div>
        )}
      </div>

      {/* Rechts: Streamer-Sidebar Toggle */}
      <button
        id="btn-001"
        aria-label="Stream-Sidebar umschalten"
        onClick={toggleSidebar}
        className={`text-[6px] font-bold uppercase tracking-[1px] px-2 py-0.5 rounded border transition-colors ${
          sidebarOpen
            ? 'border-inox-orange text-inox-orange bg-inox-orange/10'
            : 'border-[rgba(255,255,255,0.1)] text-inox-muted hover:text-inox-dim hover:border-[rgba(255,255,255,0.2)]'
        }`}
      >
        STREAM
      </button>
    </header>
  );
}

export default Header;
