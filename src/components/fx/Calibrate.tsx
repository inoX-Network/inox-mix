// Komponente: Calibrate — Quick Calibrate UI (automatische Mikrofon-Einstellung)

/** Quick Calibrate Dialog — misst Raum und empfiehlt Einstellungen */
interface CalibrateProps {
  /** Kanal-ID der kalibriert wird */
  channelId?: string;
  /** Callback wenn Kalibrierung abgeschlossen */
  onComplete?: () => void;
}

function Calibrate(_props: CalibrateProps) {
  // TODO: "Bitte 3 Sekunden still sein" Anweisung
  // TODO: Fortschrittsbalken
  // TODO: Ergebnis-Anzeige (Gain, Gate, HPF Empfehlungen)
  // TODO: "Übernehmen" / "Abbrechen" Buttons
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-4">
      {/* TODO: Calibrate-UI */}
    </div>
  );
}

export default Calibrate;
