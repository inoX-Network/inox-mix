// Types: routing — TypeScript Interfaces für Routing-Matrix (passend zu Rust)

/**
 * Routing-Eintrag (Source → Bus Verbindung)
 * (entspricht Rust: audio::routing::RoutingEntry)
 */
export interface RoutingEntry {
  /** Source-ID (z.B. "mic-1", "app-browser") */
  source_id: string;
  /** Bus-ID (A1, A2, B1, B2) */
  bus_id: string;
  /** Verbindung aktiv */
  active: boolean;
}
