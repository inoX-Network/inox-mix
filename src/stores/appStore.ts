// Store: appStore — Zustand Store für App-Level State (Tabs, System-Info, Warnungen)
import { create } from 'zustand';
import type { SystemInfo } from '../types/api';

/** Verfügbare Tabs in der Hauptnavigation */
export type AppTab = 'mixer' | 'fx' | 'routing' | 'apps' | 'settings' | 'help';

/** Tab-Definitionen mit Label und ID */
export const APP_TABS: { id: AppTab; label: string }[] = [
  { id: 'mixer', label: 'MIXER' },
  { id: 'fx', label: 'FX CHAIN' },
  { id: 'routing', label: 'ROUTING' },
  { id: 'apps', label: 'APPS' },
  { id: 'settings', label: 'SETTINGS' },
  { id: 'help', label: 'HELP' },
];

/** App-Store Interface */
interface AppState {
  /** Aktiver Tab */
  activeTab: AppTab;
  /** Stream-Sidebar sichtbar */
  sidebarOpen: boolean;
  /** System-Informationen vom Backend */
  systemInfo: SystemInfo | null;
  /** PipeWire-Warnung (falls PW nicht verfügbar) */
  pipewireWarning: string | null;

  /** Tab wechseln */
  setActiveTab: (tab: AppTab) => void;
  /** Sidebar ein-/ausblenden */
  toggleSidebar: () => void;
  /** Sidebar-Sichtbarkeit direkt setzen */
  setSidebarOpen: (open: boolean) => void;
  /** System-Info setzen (nach Backend-Abfrage) */
  setSystemInfo: (info: SystemInfo) => void;
  /** PipeWire-Warnung setzen */
  setPipewireWarning: (warning: string | null) => void;
}

/** Globaler App-Store */
export const useAppStore = create<AppState>((set) => ({
  activeTab: 'mixer',
  sidebarOpen: false,
  systemInfo: null,
  pipewireWarning: null,

  setActiveTab: (tab) => set({ activeTab: tab }),
  toggleSidebar: () => set((state) => ({ sidebarOpen: !state.sidebarOpen })),
  setSidebarOpen: (open) => set({ sidebarOpen: open }),
  setSystemInfo: (info) => set({ systemInfo: info }),
  setPipewireWarning: (warning) => set({ pipewireWarning: warning }),
}));
