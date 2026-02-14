// Store: fxStore — Zustand Store für FX-Chain
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { FxModuleInfo, FxModuleType } from '../types/fx';

interface FxState {
  /** FX-Module (Phase 1: Global, später pro Strip) */
  modules: FxModuleInfo[];
  /** Lade-Status */
  loading: boolean;
  /** Fehler-Message */
  error: string | null;

  // Actions
  /** Module vom Backend laden */
  loadFxChain: () => Promise<void>;
  /** Parameter setzen */
  setParam: (moduleType: FxModuleType, paramName: string, value: number) => Promise<void>;
  /** Bypass setzen */
  setBypass: (moduleType: FxModuleType, bypass: boolean) => Promise<void>;
}

export const useFxStore = create<FxState>((set) => ({
  modules: [],
  loading: false,
  error: null,

  loadFxChain: async () => {
    set({ loading: true, error: null });
    try {
      const modules = await invoke<FxModuleInfo[]>('get_fx_chain');
      set({ modules, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  setParam: async (moduleType: FxModuleType, paramName: string, value: number) => {
    try {
      await invoke('set_fx_param', { moduleType, paramName, value });
      // Optimistic update
      set((state) => ({
        modules: state.modules.map((m) =>
          m.module_type === moduleType
            ? {
                ...m,
                params: m.params.map((p) =>
                  p[0] === paramName ? [p[0], value] : p
                ) as [string, number][],
              }
            : m
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setBypass: async (moduleType: FxModuleType, bypass: boolean) => {
    try {
      await invoke('set_fx_bypass', { moduleType, bypass });
      // Optimistic update
      set((state) => ({
        modules: state.modules.map((m) =>
          m.module_type === moduleType
            ? { ...m, enabled: !bypass }
            : m
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },
}));

