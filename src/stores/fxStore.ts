// Store: fxStore — Zustand Store für FX-Chain State

// TODO: import { create } from 'zustand'

/** FX-Chain State Interface */
interface FxState {
  /** FX-Chains pro Kanal */
  chains: Record<string, FxChainState>;
  /** Aktuell ausgewählter Kanal für FX-Panel */
  selectedChannel: string | null;
}

/** FX-Chain eines einzelnen Kanals */
interface FxChainState {
  channelId: string;
  modules: FxModuleState[];
}

/** Einzelnes FX-Modul */
interface FxModuleState {
  type: string;
  enabled: boolean;
  params: Record<string, number>;
}

// TODO: Zustand Store erstellen
// export const useFxStore = create<FxState>((set) => ({ ... }))

export type { FxState, FxChainState, FxModuleState };
