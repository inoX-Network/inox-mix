// Types: bus — TypeScript Interfaces für Output-Busse (passend zu Rust-Structs)

/**
 * Typ eines Output-Bus
 */
export type BusType = 'Physical' | 'Virtual';

/**
 * Ein Output-Bus
 * (entspricht Rust: audio::bus::OutputBus)
 */
export interface OutputBus {
  /** Bus-ID (A1, A2, B1, B2) */
  id: string;
  /** Anzeige-Name (SPEAKERS, HEADSET, STREAM, VOIP) */
  name: string;
  /** Bus-Typ (Physical oder Virtual) */
  bus_type: BusType;
  /** Zugeordnete PipeWire-Device-ID (falls vorhanden) */
  device_id: number | null;
  /** Lautstärke in dB (-50.0 bis +10.0) */
  volume_db: number;
  /** Stummschaltung aktiv */
  muted: boolean;
  /** Recording aktiv */
  recording: boolean;
}
