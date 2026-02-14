// Komponente: VoiceFX — Stimm-Effekte Auswahl (Robot, Vader, etc.)

/** Voice FX Panel mit Effekt-Buttons und Intensitäts-Regler */
interface VoiceFXProps {
  /** Aktiver Effekt */
  activeEffect?: string;
}

function VoiceFX(_props: VoiceFXProps) {
  // TODO: "VOICE FX" Header
  // TODO: Effekt-Buttons: Robot, Vader, Chipmunk, Radio, Echo
  // TODO: Intensitäts-Slider
  return (
    <div className="p-2 border-b border-[rgba(255,255,255,0.05)]">
      {/* TODO: Voice FX Inhalt */}
    </div>
  );
}

export default VoiceFX;
