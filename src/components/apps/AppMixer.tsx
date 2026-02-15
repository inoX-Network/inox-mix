// Komponente: AppMixer — Anwendungs-Mixer (per-App Lautstärke über PipeWire)
import { useState, useEffect } from 'react';
import Slider from '../mixer/Slider';

/** App-Audio-Stream aus PipeWire */
interface AudioApp {
  id: string;
  name: string;
  icon: string;
  volume: number; // 0.0 - 1.0
  muted: boolean;
  bus_id: string | null;
}

/** Application Mixer zeigt alle laufenden Apps mit Audio und deren Routing */
interface AppMixerProps {}

function AppMixer(_props: AppMixerProps) {
  const [apps, setApps] = useState<AudioApp[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Apps vom System laden (simuliert)
    loadApps();

    // Auto-Refresh alle 5 Sekunden für neue Apps
    const interval = setInterval(loadApps, 5000);
    return () => clearInterval(interval);
  }, []);

  const loadApps = async () => {
    // TODO: Echte PipeWire Streams laden
    // Simulierte Apps für Demo
    const mockApps: AudioApp[] = [
      { id: '1', name: 'Firefox', icon: '🦊', volume: 0.8, muted: false, bus_id: 'A1' },
      { id: '2', name: 'Spotify', icon: '🎵', volume: 0.6, muted: false, bus_id: 'A1' },
      { id: '3', name: 'Discord', icon: '💬', volume: 0.9, muted: false, bus_id: 'A2' },
      { id: '4', name: 'OBS Studio', icon: '🎥', volume: 1.0, muted: false, bus_id: 'B1' },
    ];

    setApps(mockApps);
    setLoading(false);
  };

  const handleVolumeChange = (appId: string, value: number) => {
    setApps((prev) =>
      prev.map((app) =>
        app.id === appId ? { ...app, volume: value } : app
      )
    );

    // TODO: Invoke backend command to set app volume
  };

  const handleMuteToggle = (appId: string) => {
    setApps((prev) =>
      prev.map((app) =>
        app.id === appId ? { ...app, muted: !app.muted } : app
      )
    );

    // TODO: Invoke backend command to mute app
  };

  const handleBusChange = (appId: string, busId: string) => {
    setApps((prev) =>
      prev.map((app) =>
        app.id === appId ? { ...app, bus_id: busId } : app
      )
    );

    // TODO: Invoke backend command to route app to bus
  };

  if (loading) {
    return (
      <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-3">
        <div className="text-center py-8 text-[5px] text-gray-500">
          Lade laufende Apps...
        </div>
      </div>
    );
  }

  return (
    <div className="bg-inox-panel border border-[rgba(255,255,255,0.05)] rounded-[5px] p-3">
      {/* Header */}
      <div className="mb-3">
        <h3 className="text-[8px] font-extrabold uppercase tracking-wider text-cyan-500">
          App Mixer
        </h3>
        <p className="text-[5px] text-gray-600 mt-0.5">
          {apps.length} Apps mit Audio-Output
        </p>
      </div>

      {/* App-Liste */}
      <div className="space-y-3">
        {apps.length === 0 ? (
          <div className="text-center py-8 text-[5px] text-gray-500">
            Keine Apps mit Audio gefunden
          </div>
        ) : (
          apps.map((app) => (
            <div
              key={app.id}
              className="p-2 bg-gray-800 border border-gray-700 rounded"
            >
              {/* App-Header */}
              <div className="flex items-center justify-between mb-2">
                <div className="flex items-center gap-2">
                  {/* Icon */}
                  <div className="text-[14px]">{app.icon}</div>

                  {/* Name */}
                  <div>
                    <div className="text-[6px] font-semibold text-gray-200">
                      {app.name}
                    </div>
                    <div className="text-[4.5px] text-gray-500">
                      Bus: {app.bus_id || 'None'}
                    </div>
                  </div>
                </div>

                {/* Mute Button */}
                <button
                  onClick={() => handleMuteToggle(app.id)}
                  className={`px-2 py-1 text-[4.5px] font-bold uppercase tracking-wide rounded transition-colors ${
                    app.muted
                      ? 'bg-inox-error text-white'
                      : 'bg-gray-700 text-gray-400 hover:bg-gray-600'
                  }`}
                >
                  {app.muted ? 'MUTED' : 'MUTE'}
                </button>
              </div>

              {/* Volume Slider */}
              <Slider
                label="VOL"
                value={app.volume}
                onChange={(value) => handleVolumeChange(app.id, value)}
                color="#00e5ff"
                unit="%"
                disabled={app.muted}
              />

              {/* Bus-Routing */}
              <div className="mt-2 flex items-center gap-1">
                <span className="text-[4.5px] text-gray-500 uppercase mr-1">
                  Route:
                </span>
                {['A1', 'A2', 'B1', 'B2'].map((busId) => (
                  <button
                    key={busId}
                    onClick={() => handleBusChange(app.id, busId)}
                    className={`px-1.5 py-0.5 text-[4px] font-bold rounded transition-colors ${
                      app.bus_id === busId
                        ? 'bg-cyan-500 text-background'
                        : 'bg-gray-700 text-gray-500 hover:bg-gray-600'
                    }`}
                  >
                    {busId}
                  </button>
                ))}
              </div>
            </div>
          ))
        )}
      </div>

      {/* Auto-Discovery Info */}
      <div className="mt-3 pt-2 border-t border-gray-800 text-center">
        <p className="text-[4.5px] text-gray-600">
          🔄 Apps werden automatisch erkannt · PipeWire Integration
        </p>
      </div>
    </div>
  );
}

export default AppMixer;
