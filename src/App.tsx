// Root-Komponente: Header + TabBar + Content Area + Sidebar Slot
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import Header from './components/layout/Header';
import TabBar from './components/layout/TabBar';
import StreamSidebar from './components/streamer/StreamSidebar';
import { useAppStore } from './stores/appStore';
import type { SystemInfo } from './types/api';

/** inoX-MIX Hauptanwendung — verwaltet Layout und Navigation */
function App() {
  const activeTab = useAppStore((s) => s.activeTab);
  const sidebarOpen = useAppStore((s) => s.sidebarOpen);
  const setSystemInfo = useAppStore((s) => s.setSystemInfo);
  const setPipewireWarning = useAppStore((s) => s.setPipewireWarning);

  /** Beim Start: System-Info laden und PipeWire-Warnung empfangen */
  useEffect(() => {
    // System-Info vom Backend laden
    invoke<SystemInfo>('get_system_info')
      .then((info) => setSystemInfo(info))
      .catch((err) => console.error('System-Info Fehler:', err));

    // PipeWire-Warnung vom Backend empfangen
    const unlistenPromise = listen<string>('pipewire-warning', (event) => {
      setPipewireWarning(event.payload);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [setSystemInfo, setPipewireWarning]);

  return (
    <div className="h-screen flex flex-col bg-inox-bg font-oxanium text-[#cccccc] overflow-hidden">
      {/* Header-Leiste */}
      <Header />

      {/* Tab-Navigation */}
      <TabBar />

      {/* PipeWire-Warnung (falls PW nicht verfügbar) */}
      <PipewireWarningBanner />

      {/* Hauptbereich: Content + optionale Sidebar */}
      <div className="flex-1 flex overflow-hidden">
        {/* Content Area — je nach aktivem Tab */}
        <main className="flex-1 overflow-auto">
          <TabContent activeTab={activeTab} />
        </main>

        {/* Stream Sidebar (270px, rechts, ein-/ausblendbar) */}
        {sidebarOpen && (
          <StreamSidebar />
        )}
      </div>
    </div>
  );
}

/** Platzhalter-Inhalt je nach aktivem Tab (wird in späteren Modulen implementiert) */
function TabContent({ activeTab }: { activeTab: string }) {
  /** Tab-Beschreibungen für Platzhalter */
  const tabInfo: Record<string, { title: string; description: string }> = {
    mixer: {
      title: 'MIXER',
      description: 'Hardware Inputs, Virtual Inputs, Signal Monitor, Master, Output Busse',
    },
    fx: {
      title: 'FX CHAIN',
      description: 'HPF, DeNoise, Gate, DeEsser, EQ, Compressor, Limiter, AutoGain',
    },
    routing: {
      title: 'ROUTING',
      description: 'Matrix-Ansicht für Input → Bus Zuweisungen',
    },
    apps: {
      title: 'APPS',
      description: 'App-spezifische Audio-Zuweisungen (Browser, Game, Discord)',
    },
    settings: {
      title: 'SETTINGS',
      description: 'Audio-Einstellungen, Geräte-Konfiguration, UI-Optionen',
    },
    help: {
      title: 'HELP',
      description: 'FAQ, Tastenkürzel, Über inoX-MIX',
    },
  };

  const info = tabInfo[activeTab] ?? { title: activeTab.toUpperCase(), description: '' };

  return (
    <div className="flex flex-col items-center justify-center h-full gap-3 opacity-30">
      <span className="text-[14px] font-extrabold tracking-[3px] text-inox-cyan uppercase">
        {info.title}
      </span>
      <span className="text-[8px] font-medium tracking-wider text-[#666]">
        {info.description}
      </span>
      <span className="text-[6px] font-bold tracking-[1px] text-[#444] uppercase mt-2">
        Modul wird in späterer Phase implementiert
      </span>
    </div>
  );
}

/** PipeWire-Warnungs-Banner (nur sichtbar wenn PW fehlt) */
function PipewireWarningBanner() {
  const pipewireWarning = useAppStore((s) => s.pipewireWarning);
  const setPipewireWarning = useAppStore((s) => s.setPipewireWarning);

  if (!pipewireWarning) return null;

  return (
    <div className="bg-inox-red/10 border-b border-inox-red/30 px-3 py-1.5 flex items-center justify-between shrink-0">
      <div className="flex items-center gap-2">
        <div className="w-[6px] h-[6px] rounded-full bg-inox-red shadow-[0_0_4px_rgba(255,23,68,0.5)]" />
        <span className="text-[7px] font-semibold text-inox-red">
          PipeWire nicht verfügbar
        </span>
        <span className="text-[6px] text-[#888]">
          {pipewireWarning}
        </span>
      </div>
      <button
        id="btn-pw-warn-dismiss"
        onClick={() => setPipewireWarning(null)}
        className="text-[6px] font-bold text-[#666] hover:text-[#999] tracking-wider uppercase transition-colors"
      >
        SCHLIESSEN
      </button>
    </div>
  );
}

export default App;
