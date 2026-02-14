// Types: master — TypeScript Interfaces für Master-Sektion (passend zu Rust)

/**
 * Master-Sektion State
 * (entspricht Rust: audio::master::MasterState)
 */
export interface MasterState {
  /** Master Volume in dB (-∞ bis +12 dB, Standard: 0 dB) */
  volume_db: number;
  /** Master Limiter Ceiling in dB (-20 bis 0 dB, Standard: -0.1 dB) */
  limiter_ceiling_db: number;
  /** DIM aktiv (sofort -20 dB bei Unterbrechungen) */
  dim: boolean;
  /** Mono-Check aktiv (Mono-Summe für Podcast-Kompatibilität) */
  mono: boolean;
  /** Talkback aktiv (Mic auf ausgewählte Busse) */
  talkback: boolean;
  /** Talkback Ziel-Busse (z.B. ["A1", "B1"]) */
  talkback_buses: string[];
}
