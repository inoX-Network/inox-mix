// Modul: audio/mixer — Input-Strips, Lautstärke-Kontrolle und Bus-Routing
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximale Anzahl an Strips (Hardware + Virtual)
const MAX_STRIPS: usize = 10;
/// Minimale Lautstärke in dB
const MIN_VOLUME_DB: f32 = -50.0;
/// Maximale Lautstärke in dB
const MAX_VOLUME_DB: f32 = 10.0;
/// Minimaler Gain in dB
const MIN_GAIN_DB: f32 = -20.0;
/// Maximaler Gain in dB
const MAX_GAIN_DB: f32 = 20.0;

/// Typ eines Input-Strips
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StripType {
    /// Hardware-Eingang (Mikrofon, Headset, Line-In)
    Hardware,
    /// Virtueller Eingang (intern)
    Virtual,
}

/// Ein Input-Strip im Mixer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputStrip {
    /// Eindeutige Strip-ID
    pub id: String,
    /// Anzeige-Name
    pub label: String,
    /// Strip-Typ (Hardware oder Virtual)
    pub strip_type: StripType,
    /// Zugeordnete PipeWire-Device-ID (falls vorhanden)
    pub device_id: Option<u32>,
    /// Lautstärke in dB (-50.0 bis +10.0)
    pub volume_db: f32,
    /// Gain in dB (-20.0 bis +20.0)
    pub gain_db: f32,
    /// Stummschaltung aktiv
    pub muted: bool,
    /// Solo-Modus aktiv
    pub solo: bool,
    /// Pan-Position (-1.0 links, 0.0 mitte, 1.0 rechts)
    pub pan: f32,
    /// FX-Chain aktiv
    pub fx_enabled: bool,
    /// Zugewiesene Bus-Ausgänge (z.B. ["A1", "A2", "B1", "B2"])
    pub bus_routing: Vec<String>,
    /// Icon-Emoji für die Anzeige
    pub icon: String,
    /// Sortier-Reihenfolge
    pub order: u32,
}

impl InputStrip {
    /// Neuen Hardware-Strip erstellen
    pub fn new_hardware(id: &str, label: &str, icon: &str, order: u32) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            strip_type: StripType::Hardware,
            device_id: None,
            volume_db: 0.0,
            gain_db: 0.0,
            muted: false,
            solo: false,
            pan: 0.0,
            fx_enabled: false,
            bus_routing: vec!["A1".to_string()],
            icon: icon.to_string(),
            order,
        }
    }

    /// Neuen Virtual-Strip erstellen
    pub fn new_virtual(id: &str, label: &str, icon: &str, order: u32) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            strip_type: StripType::Virtual,
            device_id: None,
            volume_db: 0.0,
            gain_db: 0.0,
            muted: false,
            solo: false,
            pan: 0.0,
            fx_enabled: false,
            bus_routing: vec!["B1".to_string()],
            icon: icon.to_string(),
            order,
        }
    }
}

/// Mixer-State verwaltet alle Input-Strips und deren Routing
#[derive(Debug)]
pub struct MixerState {
    /// Alle Input-Strips (Key: Strip-ID)
    strips: HashMap<String, InputStrip>,
    /// Zähler für nächste Virtual-Strip-ID
    next_virtual_id: u32,
}

impl MixerState {
    /// Neuen Mixer-State mit Standard-Strips erstellen
    ///
    /// Standard: 3 Hardware-Strips (USB MIC, HEADSET, LINE IN)
    /// und 2 Virtual-Strips (VIRTUAL 1, VIRTUAL 2)
    pub fn new() -> Self {
        let mut strips = HashMap::new();

        // 3 Standard Hardware-Strips
        let hw_strips = [
            InputStrip::new_hardware("hw-mic", "USB MIC", "🎙️", 0),
            InputStrip::new_hardware("hw-headset", "HEADSET", "🎧", 1),
            InputStrip::new_hardware("hw-line", "LINE IN", "🔌", 2),
        ];

        for strip in hw_strips {
            strips.insert(strip.id.clone(), strip);
        }

        // 2 Standard Virtual-Strips
        let virt_strips = [
            InputStrip::new_virtual("virt-1", "VIRTUAL 1", "◆", 3),
            InputStrip::new_virtual("virt-2", "VIRTUAL 2", "◇", 4),
        ];

        for strip in virt_strips {
            strips.insert(strip.id.clone(), strip);
        }

        info!("MixerState erstellt mit {} Strips", strips.len());

        Self {
            strips,
            next_virtual_id: 3,
        }
    }

    /// Alle Strips als sortierte Liste zurückgeben
    pub fn get_strips(&self) -> Vec<InputStrip> {
        let mut strips: Vec<InputStrip> = self.strips.values().cloned().collect();
        strips.sort_by_key(|s| s.order);
        strips
    }

    /// Einen Strip anhand der ID abfragen
    pub fn get_strip(&self, strip_id: &str) -> Option<&InputStrip> {
        self.strips.get(strip_id)
    }

    /// Lautstärke eines Strips setzen (in dB)
    pub fn set_volume(&mut self, strip_id: &str, volume_db: f32) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        strip.volume_db = volume_db.clamp(MIN_VOLUME_DB, MAX_VOLUME_DB);
        Ok(())
    }

    /// Gain eines Strips setzen (in dB)
    pub fn set_gain(&mut self, strip_id: &str, gain_db: f32) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        strip.gain_db = gain_db.clamp(MIN_GAIN_DB, MAX_GAIN_DB);
        Ok(())
    }

    /// Strip stumm schalten / Stummschaltung aufheben
    pub fn set_mute(&mut self, strip_id: &str, muted: bool) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        strip.muted = muted;
        Ok(())
    }

    /// Solo-Modus für einen Strip setzen
    pub fn set_solo(&mut self, strip_id: &str, solo: bool) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        strip.solo = solo;
        Ok(())
    }

    /// Bus-Routing für einen Strip ändern
    pub fn set_bus_routing(
        &mut self,
        strip_id: &str,
        bus_id: &str,
        active: bool,
    ) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        if active {
            if !strip.bus_routing.contains(&bus_id.to_string()) {
                strip.bus_routing.push(bus_id.to_string());
            }
        } else {
            strip.bus_routing.retain(|b| b != bus_id);
        }
        Ok(())
    }

    /// Pan-Position eines Strips setzen (-1.0 bis 1.0)
    pub fn set_pan(&mut self, strip_id: &str, pan: f32) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        strip.pan = pan.clamp(-1.0, 1.0);
        Ok(())
    }

    /// FX-Chain für einen Strip aktivieren/deaktivieren
    pub fn set_fx_enabled(&mut self, strip_id: &str, enabled: bool) -> Result<(), String> {
        let strip = self
            .strips
            .get_mut(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        strip.fx_enabled = enabled;
        Ok(())
    }

    /// Neuen Virtual-Strip hinzufügen (max 10 Strips gesamt)
    pub fn add_virtual_strip(&mut self) -> Result<InputStrip, String> {
        if self.strips.len() >= MAX_STRIPS {
            return Err(format!(
                "Maximale Anzahl von {} Strips erreicht",
                MAX_STRIPS
            ));
        }

        let id = format!("virt-{}", self.next_virtual_id);
        let label = format!("VIRTUAL {}", self.next_virtual_id);
        let order = self.strips.len() as u32;
        let strip = InputStrip::new_virtual(&id, &label, "◇", order);

        self.strips.insert(id, strip.clone());
        self.next_virtual_id += 1;

        info!(
            "Virtual-Strip '{}' hinzugefügt ({}/{})",
            strip.label,
            self.strips.len(),
            MAX_STRIPS
        );

        Ok(strip)
    }

    /// Virtual-Strip entfernen (Hardware-Strips können nicht entfernt werden)
    pub fn remove_virtual_strip(&mut self, strip_id: &str) -> Result<(), String> {
        let strip = self
            .strips
            .get(strip_id)
            .ok_or_else(|| format!("Strip '{}' nicht gefunden", strip_id))?;

        if strip.strip_type != StripType::Virtual {
            return Err("Nur Virtual-Strips können entfernt werden".to_string());
        }

        self.strips.remove(strip_id);
        info!("Virtual-Strip '{}' entfernt", strip_id);
        Ok(())
    }

    /// Anzahl der aktuellen Strips
    pub fn strip_count(&self) -> usize {
        self.strips.len()
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
    fn test_mixer_state_new() {
        let state = MixerState::new();
        assert_eq!(state.strip_count(), 5, "Standard: 3 HW + 2 Virtual");
    }

    #[test]
    fn test_get_strips_sorted() {
        let state = MixerState::new();
        let strips = state.get_strips();
        assert_eq!(strips.len(), 5);
        // Sortierung nach order
        assert_eq!(strips[0].id, "hw-mic");
        assert_eq!(strips[1].id, "hw-headset");
        assert_eq!(strips[2].id, "hw-line");
        assert_eq!(strips[3].id, "virt-1");
        assert_eq!(strips[4].id, "virt-2");
    }

    #[test]
    fn test_set_volume() {
        let mut state = MixerState::new();
        state.set_volume("hw-mic", -10.5).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().volume_db, -10.5);
    }

    #[test]
    fn test_set_volume_clamp() {
        let mut state = MixerState::new();
        // Über Maximum → Clamp auf 10.0
        state.set_volume("hw-mic", 99.0).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().volume_db, MAX_VOLUME_DB);
        // Unter Minimum → Clamp auf -50.0
        state.set_volume("hw-mic", -100.0).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().volume_db, MIN_VOLUME_DB);
    }

    #[test]
    fn test_set_volume_not_found() {
        let mut state = MixerState::new();
        let result = state.set_volume("nonexistent", 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_gain() {
        let mut state = MixerState::new();
        state.set_gain("hw-mic", 5.0).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().gain_db, 5.0);
    }

    #[test]
    fn test_set_gain_clamp() {
        let mut state = MixerState::new();
        state.set_gain("hw-mic", 50.0).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().gain_db, MAX_GAIN_DB);
    }

    #[test]
    fn test_set_mute() {
        let mut state = MixerState::new();
        state.set_mute("hw-mic", true).unwrap();
        assert!(state.get_strip("hw-mic").unwrap().muted);
        state.set_mute("hw-mic", false).unwrap();
        assert!(!state.get_strip("hw-mic").unwrap().muted);
    }

    #[test]
    fn test_set_solo() {
        let mut state = MixerState::new();
        state.set_solo("hw-mic", true).unwrap();
        assert!(state.get_strip("hw-mic").unwrap().solo);
    }

    #[test]
    fn test_bus_routing_add() {
        let mut state = MixerState::new();
        // hw-mic hat standardmäßig nur A1
        state.set_bus_routing("hw-mic", "B1", true).unwrap();
        let strip = state.get_strip("hw-mic").unwrap();
        assert!(strip.bus_routing.contains(&"A1".to_string()));
        assert!(strip.bus_routing.contains(&"B1".to_string()));
    }

    #[test]
    fn test_bus_routing_remove() {
        let mut state = MixerState::new();
        state.set_bus_routing("hw-mic", "A1", false).unwrap();
        let strip = state.get_strip("hw-mic").unwrap();
        assert!(!strip.bus_routing.contains(&"A1".to_string()));
    }

    #[test]
    fn test_bus_routing_no_duplicate() {
        let mut state = MixerState::new();
        // A1 ist schon vorhanden → sollte nicht doppelt hinzugefügt werden
        state.set_bus_routing("hw-mic", "A1", true).unwrap();
        let strip = state.get_strip("hw-mic").unwrap();
        assert_eq!(strip.bus_routing.iter().filter(|b| *b == "A1").count(), 1);
    }

    #[test]
    fn test_set_pan() {
        let mut state = MixerState::new();
        state.set_pan("hw-mic", -0.5).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().pan, -0.5);
    }

    #[test]
    fn test_set_pan_clamp() {
        let mut state = MixerState::new();
        state.set_pan("hw-mic", 5.0).unwrap();
        assert_eq!(state.get_strip("hw-mic").unwrap().pan, 1.0);
    }

    #[test]
    fn test_set_fx_enabled() {
        let mut state = MixerState::new();
        state.set_fx_enabled("hw-mic", true).unwrap();
        assert!(state.get_strip("hw-mic").unwrap().fx_enabled);
    }

    #[test]
    fn test_add_virtual_strip() {
        let mut state = MixerState::new();
        assert_eq!(state.strip_count(), 5);

        let strip = state.add_virtual_strip().unwrap();
        assert_eq!(strip.strip_type, StripType::Virtual);
        assert_eq!(strip.label, "VIRTUAL 3");
        assert_eq!(state.strip_count(), 6);
    }

    #[test]
    fn test_add_virtual_strip_max_limit() {
        let mut state = MixerState::new();
        // 5 Standard + 5 weitere = 10 = MAX
        for _ in 0..5 {
            state.add_virtual_strip().unwrap();
        }
        assert_eq!(state.strip_count(), MAX_STRIPS);

        // 11. Strip → Fehler
        let result = state.add_virtual_strip();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Maximale Anzahl"));
    }

    #[test]
    fn test_remove_virtual_strip() {
        let mut state = MixerState::new();
        state.remove_virtual_strip("virt-1").unwrap();
        assert_eq!(state.strip_count(), 4);
        assert!(state.get_strip("virt-1").is_none());
    }

    #[test]
    fn test_remove_hardware_strip_fails() {
        let mut state = MixerState::new();
        let result = state.remove_virtual_strip("hw-mic");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Nur Virtual-Strips"));
    }

    #[test]
    fn test_remove_nonexistent_strip() {
        let mut state = MixerState::new();
        let result = state.remove_virtual_strip("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_hardware_strip_defaults() {
        let strip = InputStrip::new_hardware("test", "TEST", "🎙️", 0);
        assert_eq!(strip.strip_type, StripType::Hardware);
        assert_eq!(strip.volume_db, 0.0);
        assert_eq!(strip.gain_db, 0.0);
        assert!(!strip.muted);
        assert!(!strip.solo);
        assert_eq!(strip.pan, 0.0);
        assert!(!strip.fx_enabled);
        assert_eq!(strip.bus_routing, vec!["A1".to_string()]);
    }

    #[test]
    fn test_virtual_strip_defaults() {
        let strip = InputStrip::new_virtual("test", "TEST", "◆", 0);
        assert_eq!(strip.strip_type, StripType::Virtual);
        assert_eq!(strip.bus_routing, vec!["B1".to_string()]);
    }

    #[test]
    fn test_strip_serialize() {
        let strip = InputStrip::new_hardware("hw-mic", "USB MIC", "🎙️", 0);
        let json = serde_json::to_string(&strip);
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("USB MIC"));
        assert!(json_str.contains("Hardware"));
    }

    #[test]
    fn test_db_to_linear() {
        // 0 dB = 1.0 linear
        assert!((db_to_linear(0.0) - 1.0).abs() < 0.001);
        // -6 dB ≈ 0.5 linear
        assert!((db_to_linear(-6.0) - 0.501).abs() < 0.01);
        // +6 dB ≈ 2.0 linear
        assert!((db_to_linear(6.0) - 1.995).abs() < 0.01);
    }

    #[test]
    fn test_linear_to_db() {
        // 1.0 linear = 0 dB
        assert!((linear_to_db(1.0) - 0.0).abs() < 0.001);
        // 0.0 linear = MIN_VOLUME_DB
        assert_eq!(linear_to_db(0.0), MIN_VOLUME_DB);
        // Negative Werte = MIN_VOLUME_DB
        assert_eq!(linear_to_db(-0.5), MIN_VOLUME_DB);
    }
}
