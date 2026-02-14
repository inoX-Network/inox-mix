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

/** System-Informationen (Rückgabe von get_system_info Tauri Command) */
export interface SystemInfo {
  /** App-Version (aus Cargo.toml) */
  app_version: string;
  /** PipeWire-Version */
  pipewire_version: string;
  /** PipeWire läuft */
  pipewire_running: boolean;
  /** Aktuelle Sample-Rate in Hz */
  sample_rate: number;
  /** Aktuelle Buffer-Größe in Samples */
  buffer_size: number;
  /** Betriebssystem */
  os: string;
  /** CPU-Architektur */
  arch: string;
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
