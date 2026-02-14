// Modul: routing — Audio-Routing Matrix (Input/App → Output Bus)
//
// Verwaltet die Kreuzmatrix welche Audio-Quellen auf welche Output-Busse geroutet werden
// SPEC: 06-routing-matrix

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::audio::pipewire;

/// Routing-Eintrag (Source → Bus Verbindung)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingEntry {
    /// Source-ID (z.B. "mic-1", "app-browser", "app-discord")
    pub source_id: String,
    /// Bus-ID (A1, A2, B1, B2)
    pub bus_id: String,
    /// Verbindung aktiv
    pub active: bool,
}

/// Routing-Manager verwaltet die Audio-Routing Matrix
pub struct RoutingManager {
    /// Routing-Matrix: (source_id, bus_id) → active
    matrix: HashMap<(String, String), bool>,
}

impl RoutingManager {
    /// Neuer Routing-Manager mit leerer Matrix
    pub fn new() -> Self {
        log::info!("RoutingManager::new() — Erstelle leere Routing-Matrix");
        Self {
            matrix: HashMap::new(),
        }
    }

    /// Routing-Matrix als Liste abrufen
    pub fn get_routing_matrix(&self) -> Vec<RoutingEntry> {
        self.matrix
            .iter()
            .map(|((source_id, bus_id), &active)| RoutingEntry {
                source_id: source_id.clone(),
                bus_id: bus_id.clone(),
                active,
            })
            .collect()
    }

    /// Routing setzen (Verbindung aktivieren/deaktivieren)
    pub fn set_routing(&mut self, source_id: &str, bus_id: &str, active: bool) -> Result<(), String> {
        // Validierung: Bus-ID muss A1, A2, B1 oder B2 sein
        if !["A1", "A2", "B1", "B2"].contains(&bus_id) {
            return Err(format!("Ungültige Bus-ID: {}", bus_id));
        }

        // Validierung: Source-ID darf nicht leer sein
        if source_id.is_empty() {
            return Err("Source-ID darf nicht leer sein".to_string());
        }

        let key = (source_id.to_string(), bus_id.to_string());

        if active {
            // Phase 2: PipeWire Link erstellen
            pipewire::create_audio_link(source_id, bus_id)?;

            // Verbindung in Matrix aktivieren (nur bei Erfolg)
            self.matrix.insert(key, true);
            log::info!("Routing aktiviert: {} → {}", source_id, bus_id);
        } else {
            // Phase 2: PipeWire Link entfernen
            pipewire::remove_audio_link(source_id, bus_id)?;

            // Verbindung aus Matrix deaktivieren (nur bei Erfolg)
            self.matrix.remove(&key);
            log::info!("Routing deaktiviert: {} → {}", source_id, bus_id);
        }

        Ok(())
    }

    /// Routing-Status abfragen (ist Source mit Bus verbunden?)
    pub fn is_routed(&self, source_id: &str, bus_id: &str) -> bool {
        let key = (source_id.to_string(), bus_id.to_string());
        self.matrix.get(&key).copied().unwrap_or(false)
    }

    /// Alle Routings für eine Source abrufen
    pub fn get_source_routing(&self, source_id: &str) -> Vec<String> {
        self.matrix
            .iter()
            .filter_map(|((src, bus), &active)| {
                if src == source_id && active {
                    Some(bus.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Alle Routings löschen (für Tests/Reset)
    pub fn clear(&mut self) {
        self.matrix.clear();
        log::info!("Routing-Matrix geleert");
    }

    /// Anzahl aktiver Routings
    pub fn routing_count(&self) -> usize {
        self.matrix.len()
    }
}

impl Default for RoutingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing_manager_new() {
        let manager = RoutingManager::new();
        assert_eq!(manager.routing_count(), 0);
    }

    #[test]
    #[ignore] // Phase 2: Benötigt laufendes PipeWire und konfigurierte Ports
    fn test_set_routing_activate() {
        let mut manager = RoutingManager::new();
        manager.set_routing("mic-1", "A1", true).unwrap();

        assert_eq!(manager.routing_count(), 1);
        assert!(manager.is_routed("mic-1", "A1"));
    }

    #[test]
    #[ignore] // Phase 2: Benötigt laufendes PipeWire und konfigurierte Ports
    fn test_set_routing_deactivate() {
        let mut manager = RoutingManager::new();
        manager.set_routing("mic-1", "A1", true).unwrap();
        manager.set_routing("mic-1", "A1", false).unwrap();

        assert_eq!(manager.routing_count(), 0);
        assert!(!manager.is_routed("mic-1", "A1"));
    }

    #[test]
    #[ignore] // Phase 2: Benötigt laufendes PipeWire und konfigurierte Ports
    fn test_set_routing_multiple() {
        let mut manager = RoutingManager::new();
        manager.set_routing("mic-1", "A1", true).unwrap();
        manager.set_routing("mic-1", "B1", true).unwrap();
        manager.set_routing("app-discord", "A2", true).unwrap();

        assert_eq!(manager.routing_count(), 3);
        assert!(manager.is_routed("mic-1", "A1"));
        assert!(manager.is_routed("mic-1", "B1"));
        assert!(manager.is_routed("app-discord", "A2"));
    }

    #[test]
    fn test_invalid_bus_id() {
        let mut manager = RoutingManager::new();
        let result = manager.set_routing("mic-1", "X1", true);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ungültige Bus-ID: X1");
    }

    #[test]
    fn test_empty_source_id() {
        let mut manager = RoutingManager::new();
        let result = manager.set_routing("", "A1", true);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Source-ID darf nicht leer sein");
    }

    #[test]
    #[ignore] // Phase 2: Benötigt laufendes PipeWire und konfigurierte Ports
    fn test_get_routing_matrix() {
        let mut manager = RoutingManager::new();
        manager.set_routing("mic-1", "A1", true).unwrap();
        manager.set_routing("mic-1", "B1", true).unwrap();

        let matrix = manager.get_routing_matrix();
        assert_eq!(matrix.len(), 2);

        // Prüfe dass beide Einträge vorhanden sind
        assert!(matrix.iter().any(|e| e.source_id == "mic-1" && e.bus_id == "A1" && e.active));
        assert!(matrix.iter().any(|e| e.source_id == "mic-1" && e.bus_id == "B1" && e.active));
    }

    #[test]
    #[ignore] // Phase 2: Benötigt laufendes PipeWire und konfigurierte Ports
    fn test_get_source_routing() {
        let mut manager = RoutingManager::new();
        manager.set_routing("mic-1", "A1", true).unwrap();
        manager.set_routing("mic-1", "B1", true).unwrap();
        manager.set_routing("app-discord", "A2", true).unwrap();

        let mic_routing = manager.get_source_routing("mic-1");
        assert_eq!(mic_routing.len(), 2);
        assert!(mic_routing.contains(&"A1".to_string()));
        assert!(mic_routing.contains(&"B1".to_string()));

        let discord_routing = manager.get_source_routing("app-discord");
        assert_eq!(discord_routing.len(), 1);
        assert!(discord_routing.contains(&"A2".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut manager = RoutingManager::new();
        // Keine set_routing() Aufrufe — nur clear() testen
        manager.clear();
        assert_eq!(manager.routing_count(), 0);
    }
}
