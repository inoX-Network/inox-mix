// Types: mixer — TypeScript Interfaces für Mixer (passend zu Rust-Structs)

/**
 * Typ eines Input-Strips
 */
export type StripType = 'Hardware' | 'Virtual';

/**
 * Ein Input-Strip im Mixer
 * (entspricht Rust: audio::mixer::InputStrip)
 */
export interface InputStrip {
  /** Eindeutige Strip-ID */
  id: string;
  /** Anzeige-Name */
  label: string;
  /** Strip-Typ (Hardware oder Virtual) */
  strip_type: StripType;
  /** Zugeordnete PipeWire-Device-ID (falls vorhanden) */
  device_id: number | null;
  /** Lautstärke in dB (-50.0 bis +10.0) */
  volume_db: number;
  /** Gain in dB (-20.0 bis +20.0) */
  gain_db: number;
  /** Stummschaltung aktiv */
  muted: boolean;
  /** Solo-Modus aktiv */
  solo: boolean;
  /** Pan-Position (-1.0 links, 0.0 mitte, 1.0 rechts) */
  pan: number;
  /** FX-Chain aktiv */
  fx_enabled: boolean;
  /** Zugewiesene Bus-Ausgänge (z.B. ["A1", "A2", "B1", "B2"]) */
  bus_routing: string[];
  /** Icon-Emoji für die Anzeige */
  icon: string;
  /** Sortier-Reihenfolge */
  order: number;
}

/**
 * Stereo-Messwerte für einen Strip
 * (entspricht Rust: audio::metering::StripLevels)
 */
export interface StripLevels {
  /** Strip-ID */
  strip_id: string;
  /** Peak-Pegel linker Kanal in dB */
  peak_l: number;
  /** Peak-Pegel rechter Kanal in dB */
  peak_r: number;
  /** RMS-Pegel linker Kanal in dB */
  rms_l: number;
  /** RMS-Pegel rechter Kanal in dB */
  rms_r: number;
  /** Clipping erkannt */
  clipping: boolean;
}
