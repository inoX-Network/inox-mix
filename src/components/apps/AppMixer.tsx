// Komponente: AppMixer — Anwendungs-Mixer (per-App Lautstärke über PipeWire)

/** Application Mixer zeigt alle laufenden Apps mit Audio und deren Routing */
interface AppMixerProps {}

function AppMixer(_props: AppMixerProps) {
  // TODO: Liste laufender Apps mit Audio (PipeWire Streams)
  // TODO: Pro App: Icon, Name, Volume Slider, Bus-Routing
  // TODO: Auto-Erkennung neuer Apps
  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-3">
      {/* TODO: App-Mixer Inhalt */}
    </div>
  );
}

export default AppMixer;
