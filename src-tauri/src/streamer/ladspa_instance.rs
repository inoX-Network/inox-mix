// Modul: streamer/ladspa_instance — LADSPA Plugin Instance Management
//
// Plugin-Instanziierung und Real-time Audio-Processing mit Raw C-FFI
use super::ladspa_ffi::*;
use log::info;

/// LADSPA Plugin-Instanz für Audio-Processing
pub struct LadspaInstance {
    descriptor: *const LADSPA_Descriptor,
    handle: LADSPA_Handle,
    sample_rate: u64,
    activated: bool,

    // Port-Info (aus Descriptor extrahiert)
    audio_input_count: usize,
    audio_output_count: usize,
    control_input_count: usize,

    // Control-Buffer (für Parameter)
    control_values: Vec<f32>,
}

impl LadspaInstance {
    /// Neue Plugin-Instanz erstellen
    pub fn new(
        descriptor_ptr: *const LADSPA_Descriptor,
        sample_rate: u64,
    ) -> Result<Self, String> {
        if descriptor_ptr.is_null() {
            return Err("Null-Descriptor".to_string());
        }

        let descriptor = unsafe { &*descriptor_ptr };

        // Plugin instantiate-Funktion aufrufen
        let handle = if let Some(instantiate_fn) = descriptor.instantiate {
            unsafe { instantiate_fn(descriptor, sample_rate as std::os::raw::c_ulong) }
        } else {
            return Err("Plugin hat keine instantiate-Funktion".to_string());
        };

        if handle.is_null() {
            return Err("Instanziierung fehlgeschlagen (null handle)".to_string());
        }

        // Port-Typen analysieren
        let port_count = descriptor.PortCount as usize;
        let mut audio_input_count = 0;
        let mut audio_output_count = 0;
        let mut control_input_count = 0;

        for i in 0..port_count {
            let port_descriptor = unsafe { *descriptor.PortDescriptors.add(i) };

            let is_audio = (port_descriptor & LADSPA_PORT_AUDIO) != 0;
            let is_control = (port_descriptor & LADSPA_PORT_CONTROL) != 0;
            let is_input = (port_descriptor & LADSPA_PORT_INPUT) != 0;

            if is_audio && is_input {
                audio_input_count += 1;
            } else if is_audio && !is_input {
                audio_output_count += 1;
            } else if is_control && is_input {
                control_input_count += 1;
            }
        }

        // Default Control-Werte
        let control_values = vec![0.0f32; control_input_count];

        info!(
            "LADSPA Plugin instanziiert: {} Audio-In, {} Audio-Out, {} Control-In",
            audio_input_count, audio_output_count, control_input_count
        );

        Ok(Self {
            descriptor: descriptor_ptr,
            handle,
            sample_rate,
            activated: false,
            audio_input_count,
            audio_output_count,
            control_input_count,
            control_values,
        })
    }

    /// Plugin aktivieren (für RT-Processing)
    pub fn activate(&mut self) -> Result<(), String> {
        if self.activated {
            return Ok(());
        }

        let descriptor = unsafe { &*self.descriptor };

        if let Some(activate_fn) = descriptor.activate {
            unsafe {
                activate_fn(self.handle);
            }
        }

        self.activated = true;
        Ok(())
    }

    /// Plugin deaktivieren
    pub fn deactivate(&mut self) -> Result<(), String> {
        if !self.activated {
            return Ok(());
        }

        let descriptor = unsafe { &*self.descriptor };

        if let Some(deactivate_fn) = descriptor.deactivate {
            unsafe {
                deactivate_fn(self.handle);
            }
        }

        self.activated = false;
        Ok(())
    }

    /// Control-Parameter setzen
    pub fn set_control(&mut self, index: usize, value: f32) -> Result<(), String> {
        if index >= self.control_input_count {
            return Err(format!(
                "Control-Index {} ungültig (max: {})",
                index,
                self.control_input_count - 1
            ));
        }

        self.control_values[index] = value;
        Ok(())
    }

    /// Audio verarbeiten (mono)
    pub fn process_mono(&mut self, input: &[f32], output: &mut [f32]) -> Result<(), String> {
        if self.audio_input_count == 0 || self.audio_output_count == 0 {
            return Err("Plugin hat keine Audio-I/O Ports".to_string());
        }

        let len = input.len().min(output.len());

        // Buffers vorbereiten
        let mut input_buf = input[..len].to_vec();
        let mut output_buf = vec![0.0f32; len];

        let descriptor = unsafe { &*self.descriptor };

        // Ports connecten
        let mut audio_in_idx = 0;
        let mut audio_out_idx = 0;
        let mut control_in_idx = 0;

        for i in 0..(descriptor.PortCount as usize) {
            let port_descriptor = unsafe { *descriptor.PortDescriptors.add(i) };

            let is_audio = (port_descriptor & LADSPA_PORT_AUDIO) != 0;
            let is_control = (port_descriptor & LADSPA_PORT_CONTROL) != 0;
            let is_input = (port_descriptor & LADSPA_PORT_INPUT) != 0;

            if let Some(connect_fn) = descriptor.connect_port {
                if is_audio && is_input && audio_in_idx == 0 {
                    // Erste Audio-Input connecten
                    unsafe {
                        connect_fn(self.handle, i as std::os::raw::c_ulong, input_buf.as_mut_ptr() as *mut _);
                    }
                    audio_in_idx += 1;
                } else if is_audio && !is_input && audio_out_idx == 0 {
                    // Erste Audio-Output connecten
                    unsafe {
                        connect_fn(self.handle, i as std::os::raw::c_ulong, output_buf.as_mut_ptr() as *mut _);
                    }
                    audio_out_idx += 1;
                } else if is_control && is_input && control_in_idx < self.control_values.len() {
                    // Control-Input connecten
                    unsafe {
                        connect_fn(
                            self.handle,
                            i as std::os::raw::c_ulong,
                            &self.control_values[control_in_idx] as *const f32 as *mut _,
                        );
                    }
                    control_in_idx += 1;
                }
            }
        }

        // Aktivieren falls nötig
        if !self.activated {
            self.activate()?;
        }

        // Audio verarbeiten (run)
        if let Some(run_fn) = descriptor.run {
            unsafe {
                run_fn(self.handle, len as std::os::raw::c_ulong);
            }
        } else {
            return Err("Plugin hat keine run-Funktion".to_string());
        }

        // Output kopieren
        output[..len].copy_from_slice(&output_buf[..len]);

        Ok(())
    }

    /// Audio verarbeiten (stereo)
    pub fn process_stereo(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        output_l: &mut [f32],
        output_r: &mut [f32],
    ) -> Result<(), String> {
        if self.audio_input_count < 2 || self.audio_output_count < 2 {
            // Fallback: Mono-Processing
            self.process_mono(input_l, output_l)?;
            output_r.copy_from_slice(output_l);
            return Ok(());
        }

        // TODO: Echtes Stereo-Processing
        // Für jetzt: Dual-Mono
        self.process_mono(input_l, output_l)?;
        self.process_mono(input_r, output_r)?;

        Ok(())
    }
}

impl Drop for LadspaInstance {
    fn drop(&mut self) {
        // Deaktivieren
        let _ = self.deactivate();

        // Cleanup-Funktion aufrufen
        let descriptor = unsafe { &*self.descriptor };
        if let Some(cleanup_fn) = descriptor.cleanup {
            unsafe {
                cleanup_fn(self.handle);
            }
        }
    }
}

// Safety: LadspaInstance ist Send/Sync wenn handle korrekt verwaltet wird
unsafe impl Send for LadspaInstance {}
unsafe impl Sync for LadspaInstance {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::streamer::ladspa_loader::LadspaLoader;

    #[test]
    fn test_plugin_instance_creation() {
        let mut loader = LadspaLoader::new();
        if loader.scan_plugins().is_ok() && loader.plugin_count() > 0 {
            let labels = loader.list_plugins();
            if let Some(label) = labels.first() {
                if let Some(plugin) = loader.find_plugin(label) {
                    match LadspaInstance::new(plugin.descriptor_ptr, 48000) {
                        Ok(mut instance) => {
                            println!("✓ Plugin instanziiert");
                            let _ = instance.activate();
                            println!("✓ Plugin aktiviert");
                        }
                        Err(e) => {
                            println!("⚠ Fehler: {}", e);
                        }
                    }
                }
            }
        } else {
            println!("⚠ Keine LADSPA-Plugins für Test");
        }
    }
}
