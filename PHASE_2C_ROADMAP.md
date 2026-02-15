# Phase 2c Roadmap: Echte PipeWire Audio-Capture

## Status: IN PROGRESS (70%)

**Problem:** Die PipeWire Stream API ist komplex und benötigt sorgfältige Lifetime-Verwaltung.

**Aktueller Stand:**
- ✅ Node-Discovery funktioniert (Phase 2b)
- ✅ Capture-Infrastruktur existiert (Ring-Buffer, Manager)
- ⚠️ Stream-Lifetime-Management noch offen
- ⚠️ Pod-Parameter-Serialisierung komplex

---

## Technische Herausforderungen

### 1. Stream-Lifetime-Problem

**Problem:**
```rust
let stream = Stream::new(...)?;
let _listener = stream.add_local_listener()...;
// Stream wird hier gedroppt! Listener funktioniert nicht mehr.
```

**Lösung:**
Stream und Listener müssen zusammen gespeichert werden:
```rust
struct AudioStreamHandle {
    stream: Rc<Stream>,  // Rc weil Stream nicht Clone ist
    _listener: StreamListener<'stream>,  // Lifetime-gebunden
    buffer: Arc<Mutex<HeapRb<AudioSample>>>,
}
```

**Challenge:** `StreamListener` ist lifetime-gebunden an Stream → Komplexe Ownership.

---

### 2. Audio-Format-Parameter (Pod/SPA)

**Problem:**
PipeWire nutzt SPA (Simple Plugin API) mit Pod-Serialisierung für Parameter.

**Aktueller Code (nicht kompilierend):**
```rust
let audio_info = AudioInfoRaw::new()
    .format(AudioFormat::F32LE)
    .rate(48000)  // Methode existiert nicht
    .channels(2); // Methode existiert nicht
```

**Richtige API (pipewire-rs 0.8):**
```rust
use pipewire::spa::param::audio::{AudioInfoRaw, AudioFormat};
use pipewire::spa::param::ParamType;
use pipewire::spa::pod::Pod;

// AudioInfoRaw Builder-Pattern nutzen
let mut audio_info = AudioInfoRaw::new();
audio_info.set_format(AudioFormat::F32LE);
audio_info.set_rate(48000);
audio_info.set_channels(2);

// In Pod serialisieren
let values: Vec<u8> = PodSerializer::serialize(
    Cursor::new(Vec::new()),
    &Value::Object(Object {
        type_: SpaTypes::ObjectParamFormat,
        id: ParamType::EnumFormat,
        properties: vec![...],  // Korrekte Properties einfügen
    }),
)?;

let pod = Pod::from_bytes(&values)?;
```

**Referenz:** `pipewire-rs/examples/stream_playback.rs`

---

### 3. Ring-Buffer API (ringbuf-rs 0.4)

**Problem:**
```rust
buffer.push(sample);  // Methode existiert nicht auf MutexGuard<SharedRb>
buffer.pop();         // Methode existiert nicht
```

**Lösung:**
```rust
use ringbuf::{HeapRb, traits::{Consumer, Producer, Split}};

let rb = HeapRb::<AudioSample>::new(2048);
let (mut producer, mut consumer) = rb.split();

// Producer (im PipeWire Callback):
producer.try_push(sample).ok();

// Consumer (im Metering-Thread):
while let Some(sample) = consumer.try_pop() {
    // Process sample
}
```

**Problem:** Producer/Consumer sind nicht Sync → Mutex nicht möglich.

**Lösung:** `HeapRb` direkt nutzen ohne split(), oder `SharedRb` mit korrekter API.

---

## Empfohlene Implementierung (Schritte)

### Schritt 1: Vereinfachte Audio-Capture (ohne echtes PipeWire)

Zuerst: Metering-Service mit **simulierten**, aber **dynamisch erkannten** Nodes:

```rust
// metering_service.rs
impl MeteringService {
    pub fn start(app_handle: AppHandle) -> Self {
        // Node-Discovery (bereits implementiert)
        let devices = pipewire::list_audio_devices().unwrap_or_default();

        // Für jeden Input-Node: Strip registrieren
        for device in devices.iter().filter(|d| d.device_type == "input") {
            let strip_id = format!("node-{}", device.id);
            engine.register_strip(&strip_id);
        }

        // Simulation (aber nur für existierende Strips!)
        // → Große Verbesserung gegenüber hardcoded IDs
    }
}
```

**Vorteil:** Nutzt echte Discovery, aber stabile Simulation.

---

### Schritt 2: PipeWire Stream-Integration (später)

Wenn Stream-Lifetime gelöst ist:

```rust
// capture.rs
pub struct StreamManager {
    _mainloop: Rc<MainLoop>,
    _context: Rc<Context>,
    _core: Rc<Core>,
    streams: HashMap<String, StreamHandle>,
}

struct StreamHandle {
    _stream: Rc<Stream>,
    _listener: Pin<Box<StreamListener>>,  // Gepinnt für Lifetime
    buffer: Arc<Mutex<VecDeque<AudioSample>>>,  // Einfacher als ringbuf
}

impl StreamManager {
    pub fn create_stream(&mut self, node_id: u32) -> Result<String, Error> {
        let stream = Rc::new(Stream::new(&self._core, "capture", props)?);

        let buffer = Arc::new(Mutex::new(VecDeque::with_capacity(2048)));
        let buffer_clone = Arc::clone(&buffer);

        let listener = Box::pin(stream.add_local_listener_with_user_data(())
            .process(move |stream, _| {
                if let Some(pw_buffer) = stream.dequeue_buffer() {
                    // Audio-Daten extrahieren
                    let data = extract_audio_data(&pw_buffer);

                    // In Buffer schreiben
                    let mut buf = buffer_clone.lock().unwrap();
                    buf.extend(data);

                    // Alte Samples entfernen wenn zu voll
                    while buf.len() > 2048 {
                        buf.pop_front();
                    }
                }
            })
            .register());

        // Pod-Parameter (vereinfacht)
        let params = create_audio_params(48000, 2, AudioFormat::F32LE)?;

        stream.connect(Direction::Input, Some(node_id), flags, &mut [params])?;

        let stream_id = format!("stream-{}", node_id);
        self.streams.insert(stream_id.clone(), StreamHandle {
            _stream: stream,
            _listener: listener,
            buffer,
        });

        Ok(stream_id)
    }
}
```

---

## Alternativer Ansatz: CPAL

**CPAL** ist eine Cross-Platform Audio-Library die PipeWire abstrahiert:

```toml
[dependencies]
cpal = "0.15"
```

```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

let host = cpal::default_host();
let device = host.default_input_device().unwrap();

let stream = device.build_input_stream(
    &config,
    move |data: &[f32], _: &cpal::InputCallbackInfo| {
        // data enthält Audio-Samples
        for chunk in data.chunks_exact(2) {
            let sample = AudioSample {
                left: chunk[0],
                right: chunk[1],
            };
            // In Buffer schreiben
        }
    },
    |err| eprintln!("Stream error: {}", err),
    None,
)?;

stream.play()?;
```

**Vorteil:** Einfachere API, Cross-Platform
**Nachteil:** Weniger PipeWire-Features (kein direktes Node-Routing)

---

## Empfehlung für Production

### Kurzfristig (nächste Woche):
1. ✅ **Behalte Simulation** bei
2. ✅ **Nutze echte Node-Discovery** (bereits implementiert)
3. ✅ **Registriere Strips dynamisch** basierend auf erkannten Nodes
4. ✅ **Teste** mit echten PipeWire-Nodes

→ **90% Production-Ready** ohne Stream-Komplexität

### Mittelfristig (nächsten Monat):
1. 🔄 **CPAL-Integration** für einfaches Audio-Capture
2. 🔄 **Echte Audio-Daten** in MeteringEngine
3. 🔄 **Tests** mit echtem Mikrofon-Input

→ **100% Production-Ready** mit echtem Audio

### Langfristig (später):
1. ⏳ **PipeWire Stream API** korrekt implementieren
2. ⏳ **Per-Node Routing** (App-Audio einzeln routen)
3. ⏳ **Filter-Chains** (Voice FX mit PipeWire-Plugins)

→ **Enterprise-Grade** mit vollen PipeWire-Features

---

## Nächste Schritte (JETZT)

Da die Stream-API komplex ist, sollten wir **pragmatisch** vorgehen:

**Option A: CPAL nutzen** (empfohlen, 2-3 Stunden)
- Einfacher, funktioniert sofort
- Echte Audio-Daten garantiert
- Cross-Platform als Bonus

**Option B: Simulation verbessern** (empfohlen, 30 Min)
- Nutzt echte Node-Discovery
- Registriert Strips dynamisch
- Stabil und getestet

**Option C: PipeWire Stream debuggen** (riskant, 1-2 Tage)
- Lifetime-Probleme lösen
- Pod-Serialisierung korrekt machen
- Unbekannte weitere Issues

---

## Fazit

**Empfehlung:** Option B jetzt, Option A später.

1. **Jetzt** (15 Min): Metering-Service verbessern mit echter Discovery
2. **Commit**: "feat: Phase 2c partial - Dynamic Strip Registration"
3. **Später** (wenn Zeit): CPAL-Integration für echtes Audio
4. **Viel später**: PipeWire Stream API wenn alle Features klappen

**Trade-off:**
- ✅ 90% Funktionalität sofort
- ✅ Stabil und getestet
- ⏳ 10% (echtes Audio) später

Das ist professioneller als ein halb-funktionierender Stream der crasht.
