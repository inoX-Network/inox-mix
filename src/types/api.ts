// Typen: api — API und WebSocket TypeScript Definitionen

/** Tauri Command Response */
export interface CommandResponse<T> {
  /** Erfolg */
  success: boolean;
  /** Daten bei Erfolg */
  data?: T;
  /** Fehlermeldung */
  error?: string;
}

/** System-Informationen */
export interface SystemInfo {
  /** App-Version */
  version: string;
  /** OS-Name */
  os: string;
  /** CPU-Kerne */
  cpuCores: number;
  /** RAM in MB */
  ramMb: number;
  /** PipeWire-Version */
  pipewireVersion: string;
  /** PipeWire läuft */
  pipewireRunning: boolean;
}

/** WebSocket-Nachricht */
export interface WsMessage {
  /** Nachricht-Typ */
  type: "level_update" | "state_change" | "command" | "error";
  /** Payload als JSON */
  payload: unknown;
  /** Zeitstempel */
  timestamp: number;
}

/** Update-Information */
export interface UpdateInfo {
  /** Neue Version */
  version: string;
  /** Changelog */
  notes: string;
  /** Download-URL */
  url: string;
  /** Veröffentlichungsdatum */
  date: string;
}
