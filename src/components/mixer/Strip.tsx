// Komponente: Strip — Input Channel Strip (Hardware oder Virtual)

/** Channel Strip mit Fader, VU-Meter, Knobs und Bus-Routing */
interface StripProps {
  /** Kanal-ID */
  channelId?: string;
  /** Kanal-Name */
  name?: string;
  /** Kanal-Typ: "hardware" (Cyan) oder "virtual" (Orange) */
  type?: "hardware" | "virtual";
}

function Strip(_props: StripProps) {
  // TODO: Top-Accent (2px, Farbe je nach Typ)
  // TODO: Kanal-Name Label
  // TODO: Gain Knob
  // TODO: Pan Knob
  // TODO: FX-Button
  // TODO: Bus-Buttons (A1, A2, B1, B2)
  // TODO: Mute/Solo Buttons
  // TODO: VU-Meter (Stereo)
  // TODO: Fader
  // TODO: dB-Wert Anzeige
  return (
    <div className="min-w-[56px] bg-inox-strip border border-[rgba(255,255,255,0.05)] rounded-[5px] flex flex-col">
      {/* TODO: Strip-Inhalt */}
    </div>
  );
}

export default Strip;
