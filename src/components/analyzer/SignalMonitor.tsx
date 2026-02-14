// Komponente: SignalMonitor — Haupt-Signal-Analysator mit 4 animierten Wellen

/** Signal-Monitor mit 4 animierten SVG-Wellen (2x Cyan, 2x Orange) */
interface SignalMonitorProps {
  /** Audio-Level Daten für die Wellen */
  levels?: number[];
}

function SignalMonitor(_props: SignalMonitorProps) {
  // TODO: SVG Container (ViewBox: 0 0 300 38)
  // TODO: Welle 1+2: Cyan (Hardware Inputs)
  // TODO: Welle 3+4: Orange (Virtual Inputs)
  // TODO: Animation: SVG animate auf d-Attribut, 2-4s
  // TODO: Stroke-Width: 1.3, Opacity: 0.55
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-2">
      {/* TODO: Signal-Monitor SVG */}
    </div>
  );
}

export default SignalMonitor;
