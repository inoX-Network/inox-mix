// Modul: master — Master-Sektion (Master Volume, Limiter, DIM, MONO, Talkback)
//
// Verwaltet die Master-Ausgangsstufe mit globalem Volume, Limiter und Spezialfunktionen
// SPEC: 12-master

use serde::{Deserialize, Serialize};

/// Master-Sektion Zustand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterState {
    /// Master Volume in dB (-∞ bis +12 dB, Standard: 0 dB)
    pub volume_db: f32,
    /// Master Limiter Ceiling in dB (-20 bis 0 dB, Standard: -0.1 dB)
    pub limiter_ceiling_db: f32,
    /// DIM aktiv (sofort -20 dB bei Unterbrechungen)
    pub dim: bool,
    /// Mono-Check aktiv (Mono-Summe für Podcast-Kompatibilität)
    pub mono: bool,
    /// Talkback aktiv (Mic auf ausgewählte Busse)
    pub talkback: bool,
    /// Talkback Ziel-Busse (z.B. ["A1", "B1"])
    pub talkback_buses: Vec<String>,
}

impl Default for MasterState {
    fn default() -> Self {
        Self {
            volume_db: 0.0,          // 0 dB (Unity Gain)
            limiter_ceiling_db: -0.1, // -0.1 dB (leicht unter 0 dB)
            dim: false,
            mono: false,
            talkback: false,
            talkback_buses: vec![],
        }
    }
}

/// Master-Manager verwaltet die Master-Sektion
pub struct MasterManager {
    state: MasterState,
}

impl MasterManager {
    /// Neuer Master-Manager mit Default-Werten
    pub fn new() -> Self {
        log::info!("MasterManager::new() — Erstelle Master-Sektion");
        Self {
            state: MasterState::default(),
        }
    }

    /// Master-State abrufen
    pub fn get_state(&self) -> MasterState {
        self.state.clone()
    }

    /// Master Volume setzen (in dB)
    ///
    /// # Argumente
    /// * `volume_db` - Lautstärke in dB (-∞ bis +12 dB)
    pub fn set_volume(&mut self, volume_db: f32) -> Result<(), String> {
        // Validierung: -80 dB (praktisch stumm) bis +12 dB
        if volume_db < -80.0 || volume_db > 12.0 {
            return Err(format!(
                "Master Volume außerhalb des gültigen Bereichs: {} dB (erlaubt: -80 bis +12 dB)",
                volume_db
            ));
        }

        self.state.volume_db = volume_db;
        log::debug!("Master Volume: {} dB", volume_db);
        Ok(())
    }

    /// Master Limiter Ceiling setzen (in dB)
    ///
    /// # Argumente
    /// * `ceiling_db` - Limiter Ceiling in dB (-20 bis 0 dB)
    pub fn set_limiter(&mut self, ceiling_db: f32) -> Result<(), String> {
        // Validierung: -20 dB bis 0 dB
        if ceiling_db < -20.0 || ceiling_db > 0.0 {
            return Err(format!(
                "Limiter Ceiling außerhalb des gültigen Bereichs: {} dB (erlaubt: -20 bis 0 dB)",
                ceiling_db
            ));
        }

        self.state.limiter_ceiling_db = ceiling_db;
        log::debug!("Master Limiter Ceiling: {} dB", ceiling_db);
        Ok(())
    }

    /// DIM-Funktion setzen (sofort -20 dB)
    ///
    /// # Argumente
    /// * `active` - DIM aktiv (true) oder inaktiv (false)
    pub fn set_dim(&mut self, active: bool) -> Result<(), String> {
        self.state.dim = active;
        log::info!("Master DIM: {}", if active { "ON" } else { "OFF" });
        Ok(())
    }

    /// Mono-Check setzen (Mono-Summe)
    ///
    /// # Argumente
    /// * `active` - Mono aktiv (true) oder Stereo (false)
    pub fn set_mono(&mut self, active: bool) -> Result<(), String> {
        self.state.mono = active;
        log::info!("Master MONO: {}", if active { "ON" } else { "OFF" });
        Ok(())
    }

    /// Talkback setzen (Mic auf ausgewählte Busse)
    ///
    /// # Argumente
    /// * `active` - Talkback aktiv (true) oder inaktiv (false)
    /// * `target_buses` - Ziel-Busse für Talkback (z.B. ["A1", "B1"])
    pub fn set_talkback(&mut self, active: bool, target_buses: Vec<String>) -> Result<(), String> {
        // Validierung: Bus-IDs müssen A1, A2, B1 oder B2 sein
        for bus_id in &target_buses {
            if !["A1", "A2", "B1", "B2"].contains(&bus_id.as_str()) {
                return Err(format!("Ungültige Bus-ID für Talkback: {}", bus_id));
            }
        }

        self.state.talkback = active;
        self.state.talkback_buses = target_buses.clone();
        log::info!(
            "Master TALKBACK: {} (Busse: {:?})",
            if active { "ON" } else { "OFF" },
            target_buses
        );
        Ok(())
    }

    /// Effektive Master-Lautstärke berechnen (mit DIM)
    ///
    /// Gibt die tatsächliche Lautstärke zurück unter Berücksichtigung von DIM
    pub fn get_effective_volume_db(&self) -> f32 {
        if self.state.dim {
            self.state.volume_db - 20.0 // DIM: -20 dB
        } else {
            self.state.volume_db
        }
    }
}

impl Default for MasterManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_manager_new() {
        let manager = MasterManager::new();
        let state = manager.get_state();

        assert_eq!(state.volume_db, 0.0);
        assert_eq!(state.limiter_ceiling_db, -0.1);
        assert!(!state.dim);
        assert!(!state.mono);
        assert!(!state.talkback);
        assert!(state.talkback_buses.is_empty());
    }

    #[test]
    fn test_set_volume() {
        let mut manager = MasterManager::new();

        // Gültiger Bereich
        manager.set_volume(-10.0).unwrap();
        assert_eq!(manager.get_state().volume_db, -10.0);

        manager.set_volume(6.0).unwrap();
        assert_eq!(manager.get_state().volume_db, 6.0);

        // Grenzwerte
        manager.set_volume(-80.0).unwrap();
        assert_eq!(manager.get_state().volume_db, -80.0);

        manager.set_volume(12.0).unwrap();
        assert_eq!(manager.get_state().volume_db, 12.0);
    }

    #[test]
    fn test_set_volume_invalid() {
        let mut manager = MasterManager::new();

        // Zu niedrig
        let result = manager.set_volume(-100.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("außerhalb des gültigen Bereichs"));

        // Zu hoch
        let result = manager.set_volume(20.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_limiter() {
        let mut manager = MasterManager::new();

        // Gültiger Bereich
        manager.set_limiter(-3.0).unwrap();
        assert_eq!(manager.get_state().limiter_ceiling_db, -3.0);

        // Grenzwerte
        manager.set_limiter(-20.0).unwrap();
        assert_eq!(manager.get_state().limiter_ceiling_db, -20.0);

        manager.set_limiter(0.0).unwrap();
        assert_eq!(manager.get_state().limiter_ceiling_db, 0.0);
    }

    #[test]
    fn test_set_limiter_invalid() {
        let mut manager = MasterManager::new();

        // Zu niedrig
        let result = manager.set_limiter(-30.0);
        assert!(result.is_err());

        // Zu hoch
        let result = manager.set_limiter(5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_dim() {
        let mut manager = MasterManager::new();

        manager.set_dim(true).unwrap();
        assert!(manager.get_state().dim);

        manager.set_dim(false).unwrap();
        assert!(!manager.get_state().dim);
    }

    #[test]
    fn test_set_mono() {
        let mut manager = MasterManager::new();

        manager.set_mono(true).unwrap();
        assert!(manager.get_state().mono);

        manager.set_mono(false).unwrap();
        assert!(!manager.get_state().mono);
    }

    #[test]
    fn test_set_talkback() {
        let mut manager = MasterManager::new();

        // Talkback aktivieren mit gültigen Bussen
        manager
            .set_talkback(true, vec!["A1".to_string(), "B1".to_string()])
            .unwrap();

        let state = manager.get_state();
        assert!(state.talkback);
        assert_eq!(state.talkback_buses.len(), 2);
        assert!(state.talkback_buses.contains(&"A1".to_string()));
        assert!(state.talkback_buses.contains(&"B1".to_string()));

        // Talkback deaktivieren
        manager.set_talkback(false, vec![]).unwrap();
        assert!(!manager.get_state().talkback);
    }

    #[test]
    fn test_set_talkback_invalid_bus() {
        let mut manager = MasterManager::new();

        // Ungültige Bus-ID
        let result = manager.set_talkback(true, vec!["X1".to_string()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Ungültige Bus-ID"));
    }

    #[test]
    fn test_get_effective_volume_db() {
        let mut manager = MasterManager::new();

        // Ohne DIM
        manager.set_volume(0.0).unwrap();
        assert_eq!(manager.get_effective_volume_db(), 0.0);

        manager.set_volume(-6.0).unwrap();
        assert_eq!(manager.get_effective_volume_db(), -6.0);

        // Mit DIM (-20 dB)
        manager.set_dim(true).unwrap();
        assert_eq!(manager.get_effective_volume_db(), -26.0); // -6 dB - 20 dB

        manager.set_volume(0.0).unwrap();
        assert_eq!(manager.get_effective_volume_db(), -20.0); // 0 dB - 20 dB
    }

    #[test]
    fn test_master_state_serialize() {
        let state = MasterState {
            volume_db: -3.0,
            limiter_ceiling_db: -0.5,
            dim: true,
            mono: false,
            talkback: true,
            talkback_buses: vec!["A1".to_string(), "B1".to_string()],
        };

        let json = serde_json::to_string(&state);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("-3"));
        assert!(json_str.contains("true"));
        assert!(json_str.contains("A1"));
    }
}
