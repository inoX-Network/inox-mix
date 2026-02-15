// Komponente: Header — obere Leiste mit Logo, PipeWire-Status und System-Info

import { useAppStore } from '../../stores/appStore';
import RecordingControl from './RecordingControl';
import ScenesControl from './ScenesControl';

/** Header-Leiste mit App-Name, PipeWire-Status und Audio-Parametern */
function Header() {
  const systemInfo = useAppStore((s) => s.systemInfo);
  const pipewireWarning = useAppStore((s) => s.pipewireWarning);
  const sidebarOpen = useAppStore((s) => s.sidebarOpen);
  const toggleSidebar = useAppStore((s) => s.toggleSidebar);

  /** PipeWire-Status: verbunden oder Warnung */
  const pwConnected = systemInfo?.pipewire_running && !pipewireWarning;

  return (
    <header className="flex items-center justify-between px-3 py-[5px] border-b border-[rgba(255,255,255,0.05)] shrink-0"
      style={{ background: 'rgba(0,0,0,0.6)', backdropFilter: 'blur(14px)' }}
    >
      {/* Links: Logo + Version */}
      <div className="flex items-center gap-[6px]">
        {/* PW-Status Dot */}
        <div
          className={`w-[6px] h-[6px] rounded-full ${
            pwConnected
              ? 'bg-inox-green shadow-[0_0_6px_rgba(118,255,3,0.4)]'
              : 'bg-inox-red shadow-[0_0_6px_rgba(255,23,68,0.5)]'
          }`}
          style={{ animation: 'pls 2s infinite' }}
        />
        {/* Logo mit Gradient */}
        <span
          className="text-[15px] font-extrabold tracking-[3px] uppercase"
          style={{
            background: 'linear-gradient(135deg, #00e5ff, #80deea)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
          }}
        >
          inoX-MIX
        </span>
        {/* Version-Chip */}
        <span
          className="text-[7px] font-medium tracking-[1px] px-1 py-[1px] rounded-sm"
          style={{
            border: '1px solid rgba(0,229,255,0.3)',
            color: '#00e5ff',
          }}
        >
          v{systemInfo?.app_version ?? '0.3'}
        </span>
      </div>

      {/* Mitte: Tabs — werden in TabBar gerendert, hier nur Platzhalter */}

      {/* Rechts: Audio-Info, Recording, Scenes, Stream */}
      <div className="flex items-center gap-2">
        {/* Audio-Parameter (inline wie Mockup) */}
        {systemInfo && (
          <span className="text-[6px] text-white/[0.18] tracking-[1px]">
            {(systemInfo.sample_rate / 1000).toFixed(0)}kHz / 32bit / PipeWire
          </span>
        )}
        {systemInfo && (
          <span className="text-[6px] tracking-[1px] text-inox-green">
            {((systemInfo.buffer_size / systemInfo.sample_rate) * 1000).toFixed(1)}ms
          </span>
        )}

        <RecordingControl />
        <ScenesControl />

        {/* STREAM Button */}
        <button
          id="btn-001"
          aria-label="Stream-Sidebar umschalten"
          onClick={toggleSidebar}
          className={`text-[6px] font-bold uppercase tracking-[1px] px-2 py-[2px] rounded-sm border transition-colors ${
            sidebarOpen
              ? 'border-inox-orange text-inox-orange bg-inox-orange/10'
              : 'border-[rgba(255,255,255,0.1)] text-white/[0.18] hover:text-white/[0.3] hover:border-[rgba(255,255,255,0.2)]'
          }`}
        >
          STREAM
        </button>
      </div>
    </header>
  );
}

export default Header;
