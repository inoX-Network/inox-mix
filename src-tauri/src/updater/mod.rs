// Modul: updater — Update-Manager (GitHub Releases + Tauri Updater)
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri_plugin_updater::UpdaterExt;

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
    /// Verfügbar (true wenn Update vorhanden)
    pub available: bool,
}

/// Update-Manager prüft und installiert Updates
#[derive(Debug)]
pub struct UpdateManager {
    /// Auto-Check aktiviert (aus Settings)
    auto_check_enabled: bool,
}

impl UpdateManager {
    /// Neuen Update-Manager erstellen
    pub fn new() -> Self {
        Self {
            auto_check_enabled: true, // Standard: Auto-Check an
        }
    }

    /// Auto-Check aktivieren/deaktivieren
    pub fn set_auto_check(&mut self, enabled: bool) {
        self.auto_check_enabled = enabled;
    }

    /// Auto-Check Status abfragen
    pub fn is_auto_check_enabled(&self) -> bool {
        self.auto_check_enabled
    }
}

impl Default for UpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

// Tauri Commands für Update-System

/// Auf Updates prüfen (Tauri Command)
#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<Option<UpdateInfo>, String> {
    log::info!("Prüfe auf Updates...");

    match app.updater() {
        Ok(updater) => match updater.check().await {
            Ok(update) => {
                if let Some(update) = update {
                    log::info!("Update verfügbar: v{}", update.version);

                    Ok(Some(UpdateInfo {
                        version: update.version.clone(),
                        notes: update.body.clone().unwrap_or_default(),
                        url: update.download_url.to_string(),
                        date: update.date.map(|d| d.to_string()).unwrap_or_default(),
                        available: true,
                    }))
                } else {
                    log::info!("Keine Updates verfügbar");
                    Ok(None)
                }
            }
            Err(e) => {
                log::error!("Update-Prüfung fehlgeschlagen: {}", e);
                Err(format!("Update-Prüfung fehlgeschlagen: {}", e))
            }
        },
        Err(e) => {
            log::error!("Updater nicht verfügbar: {}", e);
            Err(format!("Updater nicht verfügbar: {}", e))
        }
    }
}

/// Update herunterladen und installieren (Tauri Command)
#[tauri::command]
pub async fn install_update(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
    log::info!("Starte Update-Installation...");

    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(update) => {
                    if let Some(update) = update {
                        log::info!("Lade Update v{} herunter...", update.version);

                        // Download und Installation mit Fortschrittsanzeige
                        match update
                            .download_and_install(
                                |chunk_length, content_length| {
                                    // Fortschritt an Frontend senden
                                    if let Some(total) = content_length {
                                        let progress = (chunk_length as f64 / total as f64) * 100.0;
                                        let _ = window.emit("update-progress", progress);
                                    }
                                },
                                || {
                                    // Installation abgeschlossen
                                    log::info!("Update installiert");
                                    let _ = window.emit("update-installed", ());
                                },
                            )
                            .await
                        {
                            Ok(_) => {
                                log::info!("Update erfolgreich installiert");
                                Ok(())
                            }
                            Err(e) => {
                                log::error!("Update-Installation fehlgeschlagen: {}", e);
                                Err(format!("Installation fehlgeschlagen: {}", e))
                            }
                        }
                    } else {
                        Err("Kein Update verfügbar".to_string())
                    }
                }
                Err(e) => {
                    log::error!("Update-Prüfung fehlgeschlagen: {}", e);
                    Err(format!("Update-Prüfung fehlgeschlagen: {}", e))
                }
            }
        }
        Err(e) => {
            log::error!("Updater nicht verfügbar: {}", e);
            Err(format!("Updater nicht verfügbar: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_manager_new() {
        let manager = UpdateManager::new();
        assert!(manager.is_auto_check_enabled());
    }

    #[test]
    fn test_set_auto_check() {
        let mut manager = UpdateManager::new();

        manager.set_auto_check(false);
        assert!(!manager.is_auto_check_enabled());

        manager.set_auto_check(true);
        assert!(manager.is_auto_check_enabled());
    }

    #[test]
    fn test_update_info() {
        let info = UpdateInfo {
            version: "0.4.0".to_string(),
            notes: "Neue Features".to_string(),
            url: "https://example.com/download".to_string(),
            date: "2026-02-15".to_string(),
            available: true,
        };

        assert_eq!(info.version, "0.4.0");
        assert!(info.available);
    }
}
