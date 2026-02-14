// Typen: fx — FX-Chain TypeScript Definitionen

/** Verfügbare Effekt-Typen */
export type FxType =
  | "hpf"
  | "denoise"
  | "gate"
  | "deesser"
  | "eq"
  | "compressor"
  | "limiter"
  | "autogain";

/** Standard-Reihenfolge der FX-Chain */
export const FX_CHAIN_ORDER: FxType[] = [
  "hpf",
  "denoise",
  "gate",
  "deesser",
  "eq",
  "compressor",
  "limiter",
  "autogain",
];

/** FX-Modul Parameter Definition */
export interface FxParam {
  /** Parameter-Name */
  name: string;
  /** Aktueller Wert */
  value: number;
  /** Minimal-Wert */
  min: number;
  /** Maximal-Wert */
  max: number;
  /** Schritt-Weite */
  step: number;
  /** Einheit (dB, Hz, ms, etc.) */
  unit: string;
}

/** Kalibrierungs-Ergebnis */
export interface CalibrationResult {
  /** Empfohlener Gain in dB */
  recommendedGainDb: number;
  /** Rausch-Niveau in dB */
  noiseFloorDb: number;
  /** Empfohlener Gate-Schwellwert in dB */
  recommendedGateDb: number;
  /** Empfohlene HPF-Frequenz in Hz */
  recommendedHpfHz: number;
}
