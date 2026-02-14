// Modul: api/routes — REST API Endpoints (Tauri Commands + HTTP)

/// System-Informationen abrufen (CPU, RAM, Audio-Geräte)
pub fn get_system_info() -> Result<String, String> {
    // TODO: System-Infos sammeln und als JSON zurückgeben
    todo!("get_system_info")
}

/// Mixer-State abrufen (alle Kanäle, Volumes, Mutes)
pub fn get_mixer_state() -> Result<String, String> {
    // TODO: Aktuellen Mixer-State serialisieren
    todo!("get_mixer_state")
}

/// Lautstärke eines Kanals setzen
pub fn set_volume(_channel_id: &str, _volume_db: f32) -> Result<(), String> {
    // TODO: Volume über MixerState ändern
    todo!("set_volume")
}
