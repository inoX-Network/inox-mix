// Store: busStore — Zustand Store für Output-Busse
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { OutputBus } from '../types/bus';

interface BusState {
  /** Alle Output-Busse */
  buses: OutputBus[];
  /** Lade-Status */
  loading: boolean;
  /** Fehler-Message */
  error: string | null;

  // Actions
  /** Busse vom Backend laden */
  loadBuses: () => Promise<void>;
  /** Lautstärke setzen */
  setVolume: (busId: string, volumeDb: number) => Promise<void>;
  /** Stummschaltung setzen */
  setMute: (busId: string, muted: boolean) => Promise<void>;
}

export const useBusStore = create<BusState>((set) => ({
  buses: [],
  loading: false,
  error: null,

  loadBuses: async () => {
    set({ loading: true, error: null });
    try {
      const buses = await invoke<OutputBus[]>('get_buses');
      set({ buses, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  setVolume: async (busId: string, volumeDb: number) => {
    try {
      await invoke('set_bus_volume', { busId, volumeDb });
      // Optimistic update
      set((state) => ({
        buses: state.buses.map((b) =>
          b.id === busId ? { ...b, volume_db: volumeDb } : b
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setMute: async (busId: string, muted: boolean) => {
    try {
      await invoke('set_bus_mute', { busId, muted });
      set((state) => ({
        buses: state.buses.map((b) =>
          b.id === busId ? { ...b, muted } : b
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },
}));
