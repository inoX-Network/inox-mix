// Store: masterStore — Zustand Store für Master-Sektion
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { MasterState } from '../types/master';

interface MasterStoreState extends MasterState {
  /** Lade-Status */
  loading: boolean;
  /** Fehler-Message */
  error: string | null;

  // Actions
  /** Master-State vom Backend laden */
  loadMaster: () => Promise<void>;
  /** Master Volume setzen (in dB) */
  setVolume: (volumeDb: number) => Promise<void>;
  /** Master Limiter Ceiling setzen (in dB) */
  setLimiter: (ceilingDb: number) => Promise<void>;
  /** DIM-Funktion setzen */
  setDim: (active: boolean) => Promise<void>;
  /** Mono-Check setzen */
  setMono: (active: boolean) => Promise<void>;
  /** Talkback setzen */
  setTalkback: (active: boolean, targetBuses: string[]) => Promise<void>;
}

export const useMasterStore = create<MasterStoreState>((set) => ({
  // Initial State
  volume_db: 0.0,
  limiter_ceiling_db: -0.1,
  dim: false,
  mono: false,
  talkback: false,
  talkback_buses: [],
  loading: false,
  error: null,

  loadMaster: async () => {
    set({ loading: true, error: null });
    try {
      const state = await invoke<MasterState>('get_master');
      set({ ...state, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  setVolume: async (volumeDb: number) => {
    try {
      await invoke('set_master_volume', { volumeDb });
      // Optimistic update
      set({ volume_db: volumeDb });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setLimiter: async (ceilingDb: number) => {
    try {
      await invoke('set_master_limiter', { ceilingDb });
      // Optimistic update
      set({ limiter_ceiling_db: ceilingDb });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setDim: async (active: boolean) => {
    try {
      await invoke('set_dim', { active });
      // Optimistic update
      set({ dim: active });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setMono: async (active: boolean) => {
    try {
      await invoke('set_mono', { active });
      // Optimistic update
      set({ mono: active });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setTalkback: async (active: boolean, targetBuses: string[]) => {
    try {
      await invoke('set_talkback', { active, targetBuses });
      // Optimistic update
      set({ talkback: active, talkback_buses: targetBuses });
    } catch (err) {
      set({ error: String(err) });
    }
  },
}));
