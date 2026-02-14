# Skill: PipeWire Integration

## Virtuelle Geräte erstellen
```bash
# Loopback für Output-Bus (Sink + Source)
pw-loopback --capture-props='media.class=Audio/Sink node.name=inox_a1_speakers node.description="inoX-MIX A1 Speakers"'

# In Rust: über pipewire-rs API equivalent
```

## App-Audio Routing
- PipeWire Registry überwachen für neue Clients
- Client-Name aus Node-Properties: application.name, media.name
- Routing: pw-link source_port → sink_port
- Persistenz: Letzte Routing-Config pro App speichern

## Monitoring
- Audio-Levels: Stream per-port Monitoring
- Dropouts: xrun_count aus Driver-Properties
- Latenz: quantum / sample_rate = Latenz in Sekunden

## Filter-Chain (für FX)
```
# PipeWire Filter-Chain für LADSPA/LV2
context.modules = [
  { name = libpipewire-module-filter-chain
    args = {
      node.name = "inox_fx_chain"
      audio.channels = 1
      filter.graph = {
        nodes = [
          { type = ladspa name = gate plugin = gate_1410 }
          { type = lv2  name = eq   plugin = "http://eq10q.sf.net" }
        ]
      }
    }
  }
]
```

## Wichtig
- Alles über User-Session, KEIN Root
- PipeWire Version >= 0.3.30 benötigt
- Fallback: pipewire-pulse für Legacy-Apps
- wireplumber als Session-Manager (Standard auf allen modernen Distros)
