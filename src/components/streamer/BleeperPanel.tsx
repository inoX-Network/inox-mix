// Komponente: BleeperPanel — Profanity Bleeper Einstellungen

/** Bleeper-Panel mit Modus-Auswahl, Tone und Volume */
interface BleeperPanelProps {
  /** Bleeper scharf geschaltet */
  armed?: boolean;
}

function BleeperPanel(_props: BleeperPanelProps) {
  // TODO: "BLEEPER" Header mit Arm-Toggle (Rot wenn aktiv)
  // TODO: Modus-Buttons: Beep, Mute, Noise, Reverse, Custom
  // TODO: Tone Slider (Hz, bei Beep-Modus)
  // TODO: Volume Slider (dB)
  return (
    <div className="p-2 border-b border-[rgba(255,255,255,0.05)]">
      {/* TODO: Bleeper-Panel Inhalt */}
    </div>
  );
}

export default BleeperPanel;
