# Skill: Rust Audio Backend

## PipeWire Integration
- Crate: pipewire-rs (Rust bindings für libpipewire)
- Verbindung: pw::main_loop::MainLoop in eigenem Thread
- Nodes erstellen: pw-loopback für virtuelle Geräte
- Links verwalten: pw-link Äquivalent über API
- Registry: Überwachung neuer/entfernter Clients (App-Erkennung)

## Audio-Thread Regeln
- NIEMALS allokieren auf dem Audio-Thread (kein Vec::push, kein String::new)
- NIEMALS locken (kein Mutex::lock, nur try_lock oder lock-free)
- Ring-Buffer für Audio-Daten (crate: ringbuf)
- Crossbeam-Channel für Commands (UI → Audio)
- Atomic für einfache State-Flags

## DSP Grundlagen
- Alles in f32, Range: -1.0 bis +1.0
- Gain: sample * 10.0_f32.powf(db / 20.0)
- dB zu Linear: 10.0_f32.powf(db / 20.0)
- Linear zu dB: 20.0 * linear.log10()
- RMS: (sum_of_squares / n).sqrt()
- Peak: samples.iter().map(|s| s.abs()).max()

## FX-Chain Pattern
```rust
pub trait AudioProcessor: Send {
    fn process(&mut self, buffer: &mut [f32], sample_rate: u32);
    fn set_param(&mut self, name: &str, value: f32);
    fn bypass(&self) -> bool;
}

pub struct FxChain {
    processors: Vec<Box<dyn AudioProcessor>>,
}

impl FxChain {
    pub fn process(&mut self, buffer: &mut [f32], sr: u32) {
        for proc in &mut self.processors {
            if !proc.bypass() {
                proc.process(buffer, sr);
            }
        }
    }
}
```

## Metering Pattern
- VU-Daten in SharedState (Arc<AtomicF32>)
- Audio-Thread schreibt Peak + RMS pro Buffer
- Frontend pollt via Tauri Event (60fps Timer)

## Tauri Commands
```rust
#[tauri::command]
fn set_volume(strip_id: &str, value: f32) -> Result<(), String> { ... }

#[tauri::command]
fn get_levels() -> Result<Vec<LevelData>, String> { ... }
```

## Crates (Pflicht)
- pipewire = "0.8" (PipeWire Bindings)
- ringbuf = "0.4" (Lock-free Ring Buffer)
- crossbeam-channel = "0.5" (MPMC Channels)
- rusqlite = "0.31" (SQLite)
- serde + serde_json (Serialisierung)
- tauri = "2" (Framework)
- log + env_logger (Logging)
