// Store: mixerStore — Zustand Store für Mixer-State

// TODO: import { create } from 'zustand'

/** Mixer-State Interface */
interface MixerState {
  /** Alle Kanäle (Input Strips) */
  channels: Record<string, ChannelState>;
  /** Alle Output Busse */
  buses: Record<string, BusState>;
  /** Aktiver Tab */
  activeTab: string;
}

/** Einzelner Kanal-State */
interface ChannelState {
  id: string;
  name: string;
  volume: number;
  muted: boolean;
  solo: boolean;
  pan: number;
  fxEnabled: boolean;
  busAssignments: string[];
}

/** Output Bus State */
interface BusState {
  id: string;
  name: string;
  volume: number;
  muted: boolean;
  deviceId: string;
}

// TODO: Zustand Store erstellen
// export const useMixerStore = create<MixerState>((set) => ({ ... }))

export type { MixerState, ChannelState, BusState };
