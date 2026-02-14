// Store: mixerStore — Zustand Store für Mixer-State
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { InputStrip, StripLevels } from '../types/mixer';

interface MixerState {
  /** Alle Input-Strips */
  strips: InputStrip[];
  /** Aktuelle Level-Daten pro Strip */
  levels: Record<string, StripLevels>;
  /** Lade-Status */
  loading: boolean;
  /** Fehler-Message */
  error: string | null;

  // Actions
  /** Strips vom Backend laden */
  loadStrips: () => Promise<void>;
  /** Lautstärke setzen */
  setVolume: (stripId: string, volumeDb: number) => Promise<void>;
  /** Gain setzen */
  setGain: (stripId: string, gainDb: number) => Promise<void>;
  /** Stummschaltung setzen */
  setMute: (stripId: string, muted: boolean) => Promise<void>;
  /** Solo-Modus setzen */
  setSolo: (stripId: string, solo: boolean) => Promise<void>;
  /** Bus-Routing ändern */
  setBusRouting: (stripId: string, busId: string, active: boolean) => Promise<void>;
  /** Virtual-Strip hinzufügen */
  addVirtualStrip: () => Promise<void>;
  /** Virtual-Strip entfernen */
  removeVirtualStrip: (stripId: string) => Promise<void>;
  /** Level-Daten aktualisieren (von Tauri Event) */
  updateLevels: (levels: StripLevels) => void;
}

export const useMixerStore = create<MixerState>((set) => ({
  strips: [],
  levels: {},
  loading: false,
  error: null,

  loadStrips: async () => {
    set({ loading: true, error: null });
    try {
      const strips = await invoke<InputStrip[]>('get_strips');
      set({ strips, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  setVolume: async (stripId: string, volumeDb: number) => {
    try {
      await invoke('set_strip_volume', { stripId, volumeDb });
      // Optimistic update
      set((state) => ({
        strips: state.strips.map((s) =>
          s.id === stripId ? { ...s, volume_db: volumeDb } : s
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setGain: async (stripId: string, gainDb: number) => {
    try {
      await invoke('set_strip_gain', { stripId, gainDb });
      set((state) => ({
        strips: state.strips.map((s) =>
          s.id === stripId ? { ...s, gain_db: gainDb } : s
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setMute: async (stripId: string, muted: boolean) => {
    try {
      await invoke('set_strip_mute', { stripId, muted });
      set((state) => ({
        strips: state.strips.map((s) =>
          s.id === stripId ? { ...s, muted } : s
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setSolo: async (stripId: string, solo: boolean) => {
    try {
      await invoke('set_strip_solo', { stripId, solo });
      set((state) => ({
        strips: state.strips.map((s) =>
          s.id === stripId ? { ...s, solo } : s
        ),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  setBusRouting: async (stripId: string, busId: string, active: boolean) => {
    try {
      await invoke('set_strip_bus', { stripId, busId, active });
      set((state) => ({
        strips: state.strips.map((s) => {
          if (s.id !== stripId) return s;
          const bus_routing = active
            ? [...s.bus_routing, busId]
            : s.bus_routing.filter((b) => b !== busId);
          return { ...s, bus_routing };
        }),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  addVirtualStrip: async () => {
    try {
      const newStrip = await invoke<InputStrip>('add_virtual_strip');
      set((state) => ({ strips: [...state.strips, newStrip] }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  removeVirtualStrip: async (stripId: string) => {
    try {
      await invoke('remove_virtual_strip', { stripId });
      set((state) => ({
        strips: state.strips.filter((s) => s.id !== stripId),
      }));
    } catch (err) {
      set({ error: String(err) });
    }
  },

  updateLevels: (levels: StripLevels) => {
    set((state) => ({
      levels: {
        ...state.levels,
        [levels.strip_id]: levels,
      },
    }));
  },
}));
