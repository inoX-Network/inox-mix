// Store: routingStore — Zustand Store für Routing-Matrix
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { RoutingEntry } from '../types/routing';

interface RoutingState {
  /** Routing-Einträge */
  entries: RoutingEntry[];
  /** Lade-Status */
  loading: boolean;
  /** Fehler-Message */
  error: string | null;

  // Actions
  /** Routing-Matrix vom Backend laden */
  loadRoutingMatrix: () => Promise<void>;
  /** Routing setzen (Source → Bus Verbindung) */
  setRouting: (sourceId: string, busId: string, active: boolean) => Promise<void>;
  /** Prüfen ob Source mit Bus verbunden */
  isRouted: (sourceId: string, busId: string) => boolean;
}

export const useRoutingStore = create<RoutingState>((set, get) => ({
  entries: [],
  loading: false,
  error: null,

  loadRoutingMatrix: async () => {
    set({ loading: true, error: null });
    try {
      const entries = await invoke<RoutingEntry[]>('get_routing_matrix');
      set({ entries, loading: false });
    } catch (err) {
      set({ error: String(err), loading: false });
    }
  },

  setRouting: async (sourceId: string, busId: string, active: boolean) => {
    try {
      await invoke('set_routing', { sourceId, busId, active });
      // Optimistic update
      set((state) => {
        const existing = state.entries.findIndex(
          (e) => e.source_id === sourceId && e.bus_id === busId
        );

        if (active) {
          // Verbindung aktivieren
          if (existing >= 0) {
            // Update existing
            const newEntries = [...state.entries];
            newEntries[existing] = { source_id: sourceId, bus_id: busId, active: true };
            return { entries: newEntries };
          } else {
            // Add new
            return {
              entries: [...state.entries, { source_id: sourceId, bus_id: busId, active: true }],
            };
          }
        } else {
          // Verbindung deaktivieren (entfernen)
          return {
            entries: state.entries.filter(
              (e) => !(e.source_id === sourceId && e.bus_id === busId)
            ),
          };
        }
      });
    } catch (err) {
      set({ error: String(err) });
    }
  },

  isRouted: (sourceId: string, busId: string) => {
    const { entries } = get();
    return entries.some((e) => e.source_id === sourceId && e.bus_id === busId && e.active);
  },
}));
