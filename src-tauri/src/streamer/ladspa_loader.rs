// Modul: streamer/ladspa_loader — LADSPA Plugin Discovery & Loading
//
// Vollständige LADSPA-Integration mit Dynamic Library Loading
use super::ladspa_ffi::*;
use libloading::{Library, Symbol};
use log::{info, warn};
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// LADSPA Plugin-Pfade (Standard Linux-Locations)
const LADSPA_PATHS: &[&str] = &[
    "/usr/lib/ladspa",
    "/usr/local/lib/ladspa",
    "/usr/lib/x86_64-linux-gnu/ladspa",
    "~/.ladspa",
];

/// LADSPA Descriptor Function Type (alias)
type LadspaDescriptorFn = LADSPA_Descriptor_Function;

/// Geladenes LADSPA Plugin mit Raw Descriptor
#[derive(Clone)]
pub struct LoadedPlugin {
    pub descriptor_ptr: *const LADSPA_Descriptor,
    pub library: Arc<Library>,
    pub library_path: PathBuf,
    pub unique_id: u64,
    pub label: String,
    pub name: String,
    pub port_count: usize,
}

// Safety: LoadedPlugin ist Send wenn wir sicherstellen dass der Descriptor-Pointer gültig bleibt
unsafe impl Send for LoadedPlugin {}
unsafe impl Sync for LoadedPlugin {}

/// LADSPA Plugin-Loader mit Dynamic Library Loading
pub struct LadspaLoader {
    plugins: HashMap<String, LoadedPlugin>,
    libraries: Vec<Arc<Library>>, // Halte Libraries am Leben
    scan_paths: Vec<PathBuf>,
}

impl LadspaLoader {
    /// Neuer LADSPA-Loader erstellen
    pub fn new() -> Self {
        let scan_paths = LADSPA_PATHS
            .iter()
            .filter_map(|p| {
                let expanded = shellexpand::tilde(p);
                let path = PathBuf::from(expanded.as_ref());
                if path.exists() && path.is_dir() {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        info!("LADSPA Loader initialisiert");
        info!("Scan-Pfade: {} gefunden", scan_paths.len());
        for path in &scan_paths {
            info!("  - {:?}", path);
        }

        Self {
            plugins: HashMap::new(),
            libraries: Vec::new(),
            scan_paths,
        }
    }

    /// Alle verfügbaren LADSPA-Plugins scannen und laden
    pub fn scan_plugins(&mut self) -> Result<usize, String> {
        self.plugins.clear();
        self.libraries.clear();
        let mut total_found = 0;

        for path in self.scan_paths.clone() {
            match self.scan_directory(&path) {
                Ok(count) => {
                    total_found += count;
                    info!("✓ {} Plugins in {:?}", count, path);
                }
                Err(e) => {
                    warn!("⚠ Fehler beim Scannen von {:?}: {}", path, e);
                }
            }
        }

        info!("LADSPA Scan abgeschlossen: {} Plugins gefunden", total_found);
        Ok(total_found)
    }

    /// Verzeichnis nach LADSPA-Plugins scannen und laden
    fn scan_directory(&mut self, dir: &Path) -> Result<usize, String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("Lesefehler: {}", e))?;

        let mut count = 0;

        for entry in entries.flatten() {
            let path = entry.path();

            // Nur .so Dateien (Linux Shared Libraries)
            if path.extension().and_then(|s| s.to_str()) != Some("so") {
                continue;
            }

            // Library laden
            match self.load_library(&path) {
                Ok(plugin_count) => {
                    count += plugin_count;
                }
                Err(e) => {
                    warn!("⚠ Fehler beim Laden von {:?}: {}", path.file_name(), e);
                }
            }
        }

        Ok(count)
    }

    /// LADSPA Library laden und Plugins extrahieren
    fn load_library(&mut self, path: &Path) -> Result<usize, String> {
        // Library dynamisch laden
        let mut lib = unsafe {
            Library::new(path).map_err(|e| format!("Library-Ladefehler: {}", e))?
        };

        // ladspa_descriptor Funktion finden und als raw pointer speichern
        let descriptor_fn_ptr: LADSPA_Descriptor_Function = unsafe {
            let sym: Symbol<LADSPA_Descriptor_Function> = lib
                .get(b"ladspa_descriptor\0")
                .map_err(|e| format!("Symbol nicht gefunden: {}", e))?;
            *sym.into_raw()
        };

        let lib_arc = Arc::new(lib);
        let mut plugin_count = 0;

        // Alle Plugins in dieser Library iterieren
        for index in 0..100 {
            // Max 100 Plugins per Library
            let descriptor_ptr = unsafe { descriptor_fn_ptr(index) };

            if descriptor_ptr.is_null() {
                break; // Keine weiteren Plugins
            }

            // Descriptor auslesen
            let descriptor = unsafe { &*descriptor_ptr };

            // Label und Name extrahieren
            let label = unsafe {
                CStr::from_ptr(descriptor.Label)
                    .to_string_lossy()
                    .to_string()
            };

            let name = unsafe {
                CStr::from_ptr(descriptor.Name)
                    .to_string_lossy()
                    .to_string()
            };

            // Plugin speichern
            let plugin = LoadedPlugin {
                descriptor_ptr,
                library: lib_arc.clone(),
                library_path: path.to_path_buf(),
                unique_id: descriptor.UniqueID as u64,
                label: label.clone(),
                name: name.clone(),
                port_count: descriptor.PortCount as usize,
            };

            info!(
                "  ✓ Plugin: {} ({}) - {} Ports - ID {}",
                name, label, plugin.port_count, plugin.unique_id
            );

            self.plugins.insert(label, plugin);
            plugin_count += 1;
        }

        // Library im Speicher behalten
        self.libraries.push(lib_arc);

        Ok(plugin_count)
    }

    /// Plugin nach Label finden
    pub fn find_plugin(&self, label: &str) -> Option<&LoadedPlugin> {
        self.plugins.get(label)
    }

    /// Alle Plugin-Labels auflisten
    pub fn list_plugins(&self) -> Vec<String> {
        let mut labels: Vec<String> = self.plugins.keys().cloned().collect();
        labels.sort();
        labels
    }

    /// Anzahl gefundener Plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }

    /// Plugin-Details ausgeben (für Debugging)
    pub fn print_plugin_info(&self, label: &str) {
        if let Some(plugin) = self.plugins.get(label) {
            println!("\n=== LADSPA Plugin Info ===");
            println!("Name:       {}", plugin.name);
            println!("Label:      {}", plugin.label);
            println!("Unique ID:  {}", plugin.unique_id);
            println!("Ports:      {}", plugin.port_count);
            println!("Library:    {:?}", plugin.library_path);
            println!("==========================\n");
        }
    }
}

impl Default for LadspaLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladspa_loader_creation() {
        let loader = LadspaLoader::new();
        println!("Scan-Pfade: {}", loader.scan_paths.len());
    }

    #[test]
    fn test_scan_plugins() {
        let mut loader = LadspaLoader::new();
        match loader.scan_plugins() {
            Ok(count) => {
                println!("✓ {} LADSPA-Plugins gefunden", count);
                println!("\nGefundene Plugins:");
                for label in loader.list_plugins() {
                    if let Some(plugin) = loader.find_plugin(&label) {
                        println!("  - {} ({})", plugin.name, plugin.label);
                    }
                }
            }
            Err(e) => {
                println!("⚠ Scan-Fehler: {}", e);
                println!("  → Installiere LADSPA-Plugins: sudo apt install swh-plugins tap-plugins");
            }
        }
    }
}
