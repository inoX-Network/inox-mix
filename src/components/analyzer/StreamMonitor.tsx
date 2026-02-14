// Komponente: StreamMonitor — Stream-Sidebar Signal-Analysator

/** Kompakter Signal-Monitor für die Stream-Sidebar */
interface StreamMonitorProps {
  /** Audio-Level Daten */
  levels?: number[];
}

function StreamMonitor(_props: StreamMonitorProps) {
  // TODO: 4 kleine Wellen (kompakt für Sidebar)
  // TODO: Stream-Pegel Anzeige
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-1.5">
      {/* TODO: Stream-Monitor Inhalt */}
    </div>
  );
}

export default StreamMonitor;
