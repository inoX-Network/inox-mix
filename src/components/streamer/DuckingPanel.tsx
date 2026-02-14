// Komponente: DuckingPanel — Sidechain-Ducking Einstellungen

/** Ducking-Panel mit Amount, Attack, Release und Threshold Reglern */
interface DuckingPanelProps {
  /** Ducking aktiviert */
  enabled?: boolean;
}

function DuckingPanel(_props: DuckingPanelProps) {
  // TODO: "DUCKING" Header mit Toggle
  // TODO: Amount Slider (dB)
  // TODO: Attack Slider (ms)
  // TODO: Release Slider (ms)
  // TODO: Threshold Slider (dB)
  return (
    <div className="p-2 border-b border-[rgba(255,255,255,0.05)]">
      {/* TODO: Ducking-Panel Inhalt */}
    </div>
  );
}

export default DuckingPanel;
