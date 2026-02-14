// Store: streamStore — Zustand Store für Streamer-Features

// TODO: import { create } from 'zustand'

/** Streamer State Interface */
interface StreamState {
  /** Sidebar sichtbar */
  sidebarVisible: boolean;
  /** Live-Modus aktiv */
  isLive: boolean;
  /** Stream Master Volume in dB */
  masterVolume: number;
  /** Ducking-Einstellungen */
  ducking: DuckingState;
  /** Bleeper-Einstellungen */
  bleeper: BleeperState;
  /** Voice FX */
  voiceFx: VoiceFxState;
}

/** Ducking-State */
interface DuckingState {
  enabled: boolean;
  amount: number;
  attack: number;
  release: number;
  threshold: number;
}

/** Bleeper-State */
interface BleeperState {
  armed: boolean;
  mode: string;
  toneHz: number;
  volume: number;
}

/** Voice FX State */
interface VoiceFxState {
  activeEffect: string;
  intensity: number;
}

// TODO: Zustand Store erstellen
// export const useStreamStore = create<StreamState>((set) => ({ ... }))

export type { StreamState, DuckingState, BleeperState, VoiceFxState };
