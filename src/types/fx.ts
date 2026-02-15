// Types: fx — TypeScript Interfaces für FX-Chain (passend zu Rust)

/**
 * FX-Modul Typen (passend zu Rust FxModuleType)
 * Reihenfolge: HPF → Denoise → Gate → DeEsser → EQ → Compressor → Limiter → AutoGain
 */
export type FxModuleType =
  | 'Hpf'
  | 'Denoise'
  | 'Gate'
  | 'DeEsser'
  | 'Eq'
  | 'Compressor'
  | 'Limiter'
  | 'AutoGain';

/**
 * FX-Modul Info (vom Backend)
 * (entspricht Rust: fx::FxModuleInfo)
 */
export interface FxModuleInfo {
  /** Modul-Typ */
  module_type: FxModuleType;
  /** Enabled (nicht bypassed) */
  enabled: boolean;
  /** Parameter als [name, value] Tupel */
  params: [string, number][];
}

/**
 * FX-Parameter Metadaten (für UI)
 */
export interface FxParamMeta {
  /** Parameter-Name */
  name: string;
  /** Anzeige-Label */
  label: string;
  /** Minimal-Wert */
  min: number;
  /** Maximal-Wert */
  max: number;
  /** Standard-Wert */
  default: number;
  /** Einheit (dB, Hz, ms, %) */
  unit: string;
}

/**
 * FX-Modul Metadaten (für UI)
 */
export interface FxModuleMeta {
  /** Modul-Typ */
  type: FxModuleType;
  /** Kurz-ID (z.B. "HPF", "AI-DN", "GATE") */
  shortId: string;
  /** Voller Anzeige-Name (z.B. "Hi-Pass Filter", "AI Denoise") */
  name: string;
  /** Farbe (cyan oder orange) */
  color: 'cyan' | 'orange';
  /** Parameter-Definitionen */
  params: FxParamMeta[];
}

/**
 * FX-Modul Metadaten (Phase 1: HPF + Gate)
 */
export const FX_MODULE_META: Record<FxModuleType, FxModuleMeta> = {
  Hpf: {
    type: 'Hpf',
    shortId: 'HPF',
    name: 'Hi-Pass Filter',
    color: 'cyan',
    params: [
      {
        name: 'freq',
        label: 'Cutoff',
        min: 20,
        max: 300,
        default: 80,
        unit: 'Hz',
      },
    ],
  },
  Denoise: {
    type: 'Denoise',
    shortId: 'AI-DN',
    name: 'AI Denoise',
    color: 'orange',
    params: [
      {
        name: 'strength',
        label: 'Strength',
        min: 0,
        max: 100,
        default: 50,
        unit: '%',
      },
    ],
  },
  Gate: {
    type: 'Gate',
    shortId: 'GATE',
    name: 'Noise Gate',
    color: 'cyan',
    params: [
      {
        name: 'threshold',
        label: 'Threshold',
        min: -60,
        max: 0,
        default: -40,
        unit: 'dB',
      },
      {
        name: 'attack',
        label: 'Attack',
        min: 0.1,
        max: 50,
        default: 5,
        unit: 'ms',
      },
      {
        name: 'hold',
        label: 'Hold',
        min: 0,
        max: 500,
        default: 50,
        unit: 'ms',
      },
      {
        name: 'release',
        label: 'Release',
        min: 5,
        max: 500,
        default: 100,
        unit: 'ms',
      },
    ],
  },
  DeEsser: {
    type: 'DeEsser',
    shortId: 'DE-S',
    name: 'De-Esser',
    color: 'orange',
    params: [
      {
        name: 'freq',
        label: 'Frequency',
        min: 4000,
        max: 10000,
        default: 7000,
        unit: 'Hz',
      },
      {
        name: 'threshold',
        label: 'Threshold',
        min: -40,
        max: 0,
        default: -20,
        unit: 'dB',
      },
      {
        name: 'ratio',
        label: 'Ratio',
        min: 2,
        max: 10,
        default: 4,
        unit: ':1',
      },
    ],
  },
  Eq: {
    type: 'Eq',
    shortId: 'EQ',
    name: 'Equalizer',
    color: 'cyan',
    params: [
      {
        name: 'low_freq',
        label: 'Low Freq',
        min: 20,
        max: 500,
        default: 80,
        unit: 'Hz',
      },
      {
        name: 'low_gain',
        label: 'Low Gain',
        min: -12,
        max: 12,
        default: 0,
        unit: 'dB',
      },
      {
        name: 'low_q',
        label: 'Low Q',
        min: 0.5,
        max: 5,
        default: 1,
        unit: '',
      },
      {
        name: 'mid_freq',
        label: 'Mid Freq',
        min: 200,
        max: 5000,
        default: 1000,
        unit: 'Hz',
      },
      {
        name: 'mid_gain',
        label: 'Mid Gain',
        min: -12,
        max: 12,
        default: 0,
        unit: 'dB',
      },
      {
        name: 'mid_q',
        label: 'Mid Q',
        min: 0.5,
        max: 5,
        default: 1,
        unit: '',
      },
      {
        name: 'high_freq',
        label: 'High Freq',
        min: 2000,
        max: 20000,
        default: 8000,
        unit: 'Hz',
      },
      {
        name: 'high_gain',
        label: 'High Gain',
        min: -12,
        max: 12,
        default: 0,
        unit: 'dB',
      },
      {
        name: 'high_q',
        label: 'High Q',
        min: 0.5,
        max: 5,
        default: 1,
        unit: '',
      },
    ],
  },
  Compressor: {
    type: 'Compressor',
    shortId: 'COMP',
    name: 'Compressor',
    color: 'orange',
    params: [
      {
        name: 'threshold',
        label: 'Threshold',
        min: -60,
        max: 0,
        default: -20,
        unit: 'dB',
      },
      {
        name: 'ratio',
        label: 'Ratio',
        min: 1,
        max: 20,
        default: 4,
        unit: ':1',
      },
      {
        name: 'attack',
        label: 'Attack',
        min: 0.1,
        max: 100,
        default: 5,
        unit: 'ms',
      },
      {
        name: 'release',
        label: 'Release',
        min: 10,
        max: 1000,
        default: 100,
        unit: 'ms',
      },
    ],
  },
  Limiter: {
    type: 'Limiter',
    shortId: 'LIM',
    name: 'Limiter',
    color: 'cyan',
    params: [
      {
        name: 'ceiling',
        label: 'Ceiling',
        min: -20,
        max: 0,
        default: -0.3,
        unit: 'dB',
      },
      {
        name: 'release',
        label: 'Release',
        min: 10,
        max: 1000,
        default: 50,
        unit: 'ms',
      },
    ],
  },
  AutoGain: {
    type: 'AutoGain',
    shortId: 'A-G',
    name: 'Auto-Gain',
    color: 'orange',
    params: [
      {
        name: 'target_level',
        label: 'Target Level',
        min: -40,
        max: 0,
        default: -18,
        unit: 'dB',
      },
      {
        name: 'window',
        label: 'Window',
        min: 100,
        max: 5000,
        default: 1000,
        unit: 'ms',
      },
      {
        name: 'attack',
        label: 'Attack',
        min: 10,
        max: 1000,
        default: 100,
        unit: 'ms',
      },
      {
        name: 'release',
        label: 'Release',
        min: 100,
        max: 5000,
        default: 500,
        unit: 'ms',
      },
    ],
  },
};
