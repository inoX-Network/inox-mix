// Komponente: OutputBus — Ausgangs-Bus Strip (A1 Speakers, A2 Headset, B1 Stream, B2 VoIP)

/** Output Bus Strip mit Fader, VU-Meter und Geräte-Auswahl */
interface OutputBusProps {
  /** Bus-ID (z.B. "A1", "B1") */
  busId?: string;
  /** Bus-Name (z.B. "SPEAKERS", "STREAM") */
  name?: string;
  /** Bus-Typ: "A" (Cyan, physisch) oder "B" (Orange, virtuell) */
  type?: "A" | "B";
}

function OutputBus(_props: OutputBusProps) {
  // TODO: Bus-Name Label
  // TODO: Geräte-Auswahl Dropdown
  // TODO: VU-Meter (Stereo)
  // TODO: Fader
  // TODO: Mute-Button
  return (
    <div className="bg-inox-strip border border-[rgba(255,255,255,0.05)] rounded-[5px] p-2">
      {/* TODO: OutputBus-Inhalt */}
    </div>
  );
}

export default OutputBus;
