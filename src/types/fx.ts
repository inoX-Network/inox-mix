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
  /** Anzeige-Name (z.B. "HPF", "GATE") */
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
    name: 'HPF',
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
    name: 'AI-DN',
    color: 'orange',
    params: [],
  },
  Gate: {
    type: 'Gate',
    name: 'GATE',
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
    name: 'DE-S',
    color: 'orange',
    params: [],
  },
  Eq: {
    type: 'Eq',
    name: 'EQ',
    color: 'cyan',
    params: [],
  },
  Compressor: {
    type: 'Compressor',
    name: 'COMP',
    color: 'orange',
    params: [],
  },
  Limiter: {
    type: 'Limiter',
    name: 'LIM',
    color: 'cyan',
    params: [],
  },
  AutoGain: {
    type: 'AutoGain',
    name: 'A-G',
    color: 'orange',
    params: [],
  },
};
