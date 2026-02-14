// Modul: audio/bus — Output-Busse (A1/A2 physisch, B1/B2 virtuell)
use serde::{Deserialize, Serialize};
use log::info;
use std::collections::HashMap;

/// Minimale Lautstärke in dB
const MIN_VOLUME_DB: f32 = -50.0;
/// Maximale Lautstärke in dB
const MAX_VOLUME_DB: f32 = 10.0;

/// Typ eines Output-Bus
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BusType {
    /// Physischer Ausgang (Speakers, Headset)
    Physical,
    /// Virtueller Ausgang (Stream, VoIP)
    Virtual,
}

/// Ein Output-Bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputBus {
    /// Bus-ID (A1, A2, B1, B2)
    pub id: String,
    /// Anzeige-Name (SPEAKERS, HEADSET, STREAM, VOIP)
    pub name: String,
    /// Bus-Typ (Physical oder Virtual)
    pub bus_type: BusType,
    /// Zugeordnete PipeWire-Device-ID (falls vorhanden)
    pub device_id: Option<u32>,
    /// Lautstärke in dB (-50.0 bis +10.0)
    pub volume_db: f32,
    /// Stummschaltung aktiv
    pub muted: bool,
    /// Recording aktiv
    pub recording: bool,
}

impl OutputBus {
    /// Neuen physischen Bus erstellen
    pub fn new_physical(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            bus_type: BusType::Physical,
            device_id: None,
            volume_db: 0.0,
            muted: false,
            recording: false,
        }
    }

    /// Neuen virtuellen Bus erstellen
    pub fn new_virtual(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            bus_type: BusType::Virtual,
            device_id: None,
            volume_db: 0.0,
            muted: false,
            recording: false,
        }
    }
}

/// Bus-Manager verwaltet alle Output-Busse
#[derive(Debug)]
pub struct BusManager {
    /// Alle Output-Busse (Key: Bus-ID)
    buses: HashMap<String, OutputBus>,
}

impl BusManager {
    /// Neuen Bus-Manager mit 4 Standard-Bussen erstellen
    ///
    /// A1: SPEAKERS (Physical, Cyan)
    /// A2: HEADSET (Physical, Cyan)
    /// B1: STREAM (Virtual, Orange)
    /// B2: VOIP (Virtual, Orange)
    pub fn new() -> Self {
        let mut buses = HashMap::new();

        // A-Busse (Physical, Cyan)
        buses.insert("A1".to_string(), OutputBus::new_physical("A1", "SPEAKERS"));
        buses.insert("A2".to_string(), OutputBus::new_physical("A2", "HEADSET"));

        // B-Busse (Virtual, Orange)
        buses.insert("B1".to_string(), OutputBus::new_virtual("B1", "STREAM"));
        buses.insert("B2".to_string(), OutputBus::new_virtual("B2", "VOIP"));

        info!("BusManager erstellt mit 4 Standard-Bussen");

        Self { buses }
    }

    /// Alle Busse als sortierte Liste zurückgeben (A1, A2, B1, B2)
    pub fn get_buses(&self) -> Vec<OutputBus> {
        let mut buses: Vec<OutputBus> = self.buses.values().cloned().collect();
        buses.sort_by_key(|b| b.id.clone());
        buses
    }

    /// Einen Bus anhand der ID abfragen
    pub fn get_bus(&self, bus_id: &str) -> Option<&OutputBus> {
        self.buses.get(bus_id)
    }

    /// Lautstärke eines Bus setzen (in dB)
    pub fn set_volume(&mut self, bus_id: &str, volume_db: f32) -> Result<(), String> {
        let bus = self.buses.get_mut(bus_id)
            .ok_or_else(|| format!("Bus '{}' nicht gefunden", bus_id))?;

        bus.volume_db = volume_db.clamp(MIN_VOLUME_DB, MAX_VOLUME_DB);
        Ok(())
    }

    /// Bus stumm schalten / Stummschaltung aufheben
    pub fn set_mute(&mut self, bus_id: &str, muted: bool) -> Result<(), String> {
        let bus = self.buses.get_mut(bus_id)
            .ok_or_else(|| format!("Bus '{}' nicht gefunden", bus_id))?;

        bus.muted = muted;
        Ok(())
    }

    /// Recording für einen Bus aktivieren/deaktivieren
    pub fn set_recording(&mut self, bus_id: &str, recording: bool) -> Result<(), String> {
        let bus = self.buses.get_mut(bus_id)
            .ok_or_else(|| format!("Bus '{}' nicht gefunden", bus_id))?;

        bus.recording = recording;
        Ok(())
    }

    /// Anzahl der Busse
    pub fn bus_count(&self) -> usize {
        self.buses.len()
    }
}

/// dB-Wert in linearen Faktor umrechnen
pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

/// Linearen Faktor in dB-Wert umrechnen
pub fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0 {
        MIN_VOLUME_DB
    } else {
        20.0 * linear.log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_manager_new() {
        let manager = BusManager::new();
        assert_eq!(manager.bus_count(), 4);
    }

    #[test]
    fn test_get_buses_sorted() {
        let manager = BusManager::new();
        let buses = manager.get_buses();
        assert_eq!(buses.len(), 4);
        assert_eq!(buses[0].id, "A1");
        assert_eq!(buses[1].id, "A2");
        assert_eq!(buses[2].id, "B1");
        assert_eq!(buses[3].id, "B2");
    }

    #[test]
    fn test_get_bus() {
        let manager = BusManager::new();
        let bus = manager.get_bus("A1").unwrap();
        assert_eq!(bus.name, "SPEAKERS");
        assert_eq!(bus.bus_type, BusType::Physical);
    }

    #[test]
    fn test_set_volume() {
        let mut manager = BusManager::new();
        manager.set_volume("A1", -10.5).unwrap();
        assert_eq!(manager.get_bus("A1").unwrap().volume_db, -10.5);
    }

    #[test]
    fn test_set_volume_clamp() {
        let mut manager = BusManager::new();
        // Über Maximum → Clamp auf 10.0
        manager.set_volume("A1", 99.0).unwrap();
        assert_eq!(manager.get_bus("A1").unwrap().volume_db, MAX_VOLUME_DB);
        // Unter Minimum → Clamp auf -50.0
        manager.set_volume("A1", -100.0).unwrap();
        assert_eq!(manager.get_bus("A1").unwrap().volume_db, MIN_VOLUME_DB);
    }

    #[test]
    fn test_set_volume_not_found() {
        let mut manager = BusManager::new();
        let result = manager.set_volume("X1", 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_mute() {
        let mut manager = BusManager::new();
        manager.set_mute("A1", true).unwrap();
        assert!(manager.get_bus("A1").unwrap().muted);
        manager.set_mute("A1", false).unwrap();
        assert!(!manager.get_bus("A1").unwrap().muted);
    }

    #[test]
    fn test_set_recording() {
        let mut manager = BusManager::new();
        manager.set_recording("B1", true).unwrap();
        assert!(manager.get_bus("B1").unwrap().recording);
    }

    #[test]
    fn test_physical_bus_defaults() {
        let bus = OutputBus::new_physical("A1", "SPEAKERS");
        assert_eq!(bus.bus_type, BusType::Physical);
        assert_eq!(bus.volume_db, 0.0);
        assert!(!bus.muted);
        assert!(!bus.recording);
    }

    #[test]
    fn test_virtual_bus_defaults() {
        let bus = OutputBus::new_virtual("B1", "STREAM");
        assert_eq!(bus.bus_type, BusType::Virtual);
    }

    #[test]
    fn test_bus_serialize() {
        let bus = OutputBus::new_physical("A1", "SPEAKERS");
        let json = serde_json::to_string(&bus);
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("SPEAKERS"));
        assert!(json_str.contains("Physical"));
    }

    #[test]
    fn test_db_to_linear() {
        assert!((db_to_linear(0.0) - 1.0).abs() < 0.001);
        assert!((db_to_linear(-6.0) - 0.501).abs() < 0.01);
    }

    #[test]
    fn test_linear_to_db() {
        assert!((linear_to_db(1.0) - 0.0).abs() < 0.001);
        assert_eq!(linear_to_db(0.0), MIN_VOLUME_DB);
    }
}
