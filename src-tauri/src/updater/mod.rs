// Modul: updater — Update-Manager (GitHub Releases + Tauri Updater)
use serde::{Deserialize, Serialize};

/// Update-Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    /// Neue Version
    pub version: String,
    /// Changelog / Release Notes
    pub notes: String,
    /// Download-URL
    pub url: String,
    /// Veröffentlichungsdatum
    pub date: String,
}

/// Update-Manager prüft und installiert Updates
#[derive(Debug)]
pub struct UpdateManager {
    // TODO: Tauri Updater Plugin Referenz
    // TODO: Aktueller Update-Status
}

impl UpdateManager {
    /// Neuen Update-Manager erstellen
    pub fn new() -> Self {
        // TODO: Initialisieren
        todo!("UpdateManager::new")
    }

    /// Auf Updates prüfen
    pub fn check_for_update(&self) -> Result<Option<UpdateInfo>, Box<dyn std::error::Error>> {
        // TODO: GitHub Releases API abfragen
        todo!("UpdateManager::check_for_update")
    }

    /// Update herunterladen und installieren
    pub fn install_update(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Tauri Updater nutzen
        todo!("UpdateManager::install_update")
    }
}
