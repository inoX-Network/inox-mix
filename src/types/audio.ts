// Typen: audio — Audio-bezogene TypeScript Definitionen

/** Audio-Gerät (PipeWire Node) */
export interface AudioDevice {
  /** PipeWire Node-ID */
  id: number;
  /** Anzeige-Name */
  name: string;
  /** Typ: "source" oder "sink" */
  deviceType: "source" | "sink";
  /** Anzahl Kanäle */
  channels: number;
}

/** Metering-Daten für einen Kanal */
export interface MeterData {
  /** Kanal-ID */
  channelId: string;
  /** Peak-Pegel in dB */
  peakDb: number;
  /** RMS-Pegel in dB */
  rmsDb: number;
  /** Clipping erkannt */
  clipping: boolean;
}

/** Mixer-Kanal Konfiguration */
export interface ChannelConfig {
  /** Kanal-ID */
  id: string;
  /** Anzeige-Name */
  name: string;
  /** Kanal-Typ */
  type: "hardware" | "virtual" | "app";
  /** Zugewiesenes Audio-Gerät */
  deviceId?: number;
}

/** Audio-Konstanten */
export const AUDIO_CONSTANTS = {
  /** Standard Sample-Rate */
  SAMPLE_RATE: 48000,
  /** Standard Buffer-Größe */
  BUFFER_SIZE: 256,
  /** Bit-Tiefe (intern) */
  BIT_DEPTH: 32,
  /** Minimale Lautstärke in dB */
  MIN_DB: -50,
  /** Maximale Lautstärke in dB */
  MAX_DB: 10,
  /** VU-Meter Refresh-Rate in fps */
  METER_FPS: 60,
} as const;
