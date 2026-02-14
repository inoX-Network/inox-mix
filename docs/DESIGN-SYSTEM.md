# DESIGN-SYSTEM.md - inoX-MIX UI/Design Spezifikation

## Farbschema: Zweifarbig Cyan + Orange

### Primärfarben
| Name | Hex | Verwendung |
|------|-----|-----------|
| Cyan | #00e5ff | Hardware-Inputs, A-Busse, Master, aktive FX, Signal-Monitor (1+2) |
| Orange | #ff8c00 | Virtual-Inputs, B-Busse, Stream-Sidebar, Compressor/Limiter, Monitor (3+4) |

### Funktionale Farben (NUR wo nötig)
| Name | Hex | Verwendung |
|------|-----|-----------|
| Rot | #ff1744 | Mute, REC, Clipping, Bleeper-Armed |
| Grün | #4caf50 | Status OK (CPU, Latenz, Gate OPEN) |
| Amber | #e6a117 | VU Peak-Warnung |

### Hintergründe
| Name | Hex | Verwendung |
|------|-----|-----------|
| bg | #08090b | Body |
| panel | #0d0f13 | Panels, Sidebar |
| strip | #111318 | Channel Strips |
| border | rgba(255,255,255,0.05) | Alle Borders |

## Typografie

- Font: Oxanium (Google Fonts)
- Weights: 300 (Light), 400 (Regular), 500 (Medium), 600 (SemiBold), 700 (Bold), 800 (ExtraBold)
- Labels: 5-6px, 700, letter-spacing: 0.5-2.5px, uppercase
- Values: 7-9px, 600-700
- Headers: 10-15px, 800, letter-spacing: 2px

## Komponenten-Specs

### Fader
- Track: 2px breit, rgba(255,255,255,0.03)
- Fill: Farbig, box-shadow: 0 0 4px [color]25
- Thumb: 14×9px, linear-gradient(#484848,#282828), border: 1px solid #555

### Knob
- Größen: 16, 18, 20, 22, 24px
- Background: radial-gradient(circle at 40% 35%, #353535, #181818)
- Border: 1.5px solid #2a2a2a
- Needle: 1.5px breit, Farbe je nach Kontext

### Horizontal Slider
- Track: 5px hoch, rgba(255,255,255,0.03), border-radius: 3px
- Fill: Farbig, box-shadow: 0 0 4px [color]25
- Thumb: 6×8px, linear-gradient(#555,#333), border: 1px solid #666
- Label: links oben (4.5px), Value: rechts oben (5px)

### VU-Meter
- 13 Segmente vertikal
- Segment 0-8: Kanal-Farbe
- Segment 9-10: Amber (#e6a117)
- Segment 11-12: Rot (#ff1744)
- Segmentbreite: 3.5px, border-radius: 1px, gap: 1px

### Channel Strip
- Min-Width: 56px
- Background: var(--strip)
- Border: 1px solid var(--border)
- Border-Radius: 5px
- Top-Accent: 2px Höhe, Kanal-Farbe, opacity: 0.45
- Dock-Handle: 6 Dots (2×3), top-right, opacity 0→0.3 on hover

### Chip/Tag
- Padding: 1.5px 4px
- Font: 5px, 700, uppercase, letter-spacing: 0.4px
- Active: background + color filled
- Inactive: border only, rgba text

### Animated Wave (SVG)
- ViewBox: 0 0 300 38
- Stroke-Width: 1.3
- Opacity: 0.55
- Fill: linearGradient top→bottom, opacity 0.07
- Animation: SVG animate on d attribute, 2-4s duration

## Layout

### Mixer Tab (Hauptansicht)
```
┌──────────────────────────────────────────────────┐
│  [HW Inputs]  │  [Signal Monitor]  [Master]  │ [Virtual]  │
│  USB MIC       │  ┌──────────────┐ ┌──────┐  │ VIRT 1     │
│  HEADSET       │  │ Wave 1 Cyan  │ │ VOL  │  │ VIRT 2     │
│  LINE IN       │  │ Wave 2 Cyan  │ │ LIM  │  │ +          │
│                │  │ Wave 3 Orange│ │ VU   │  │            │
│                │  │ Wave 4 Orange│ │ Fader│  │            │
│                │  └──────────────┘ └──────┘  │            │
├──────────────────────────────────────────────────┤
│  [A1 SPEAKERS] [A2 HEADSET] [B1 STREAM] [B2 VOIP]         │
└──────────────────────────────────────────────────┘
```

### Stream Sidebar (270px, rechts)
```
┌─────────────────────┐
│ STREAMER  [LIVE]    │
├──────────┬──────────┤
│ STREAM   │ STREAM   │
│ MASTER   │ MONITOR  │
│ Fader+VU │ 4 Waves  │
├──────────┴──────────┤
│ AUDIO PROTECTION    │
│ ── DUCKING ──       │
│ [AMT][ATK][REL][THR]│
│ ─────────────────── │
│ ── BLEEPER ──       │
│ [Beep][Mute][Noise] │
│ [Reverse][Custom]   │
│ [TONE]     [VOL]    │
├─────────────────────┤
│ VOICE FX            │
│ [Robot][Vader][...]  │
├─────────────────────┤
│ SOUNDBOARD          │
│ [🔊][🥁][👏][💥][+] │
└─────────────────────┘
```
