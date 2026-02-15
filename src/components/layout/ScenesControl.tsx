// Komponente: ScenesControl — Scene Save/Load im Header
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface SceneInfo {
  id: string;
  name: string;
  created_at: number;
}

/** Scenes-Control mit Dropdown und Save-Button */
export default function ScenesControl() {
  const [scenes, setScenes] = useState<SceneInfo[]>([]);
  const [selectedScene, setSelectedScene] = useState<string>('');
  const [showSaveModal, setShowSaveModal] = useState(false);
  const [newSceneName, setNewSceneName] = useState('');

  // Scenes laden
  useEffect(() => {
    loadScenes();
  }, []);

  const loadScenes = () => {
    invoke<SceneInfo[]>('get_scenes')
      .then(setScenes)
      .catch(console.error);
  };

  // Scene laden
  const loadScene = (sceneId: string) => {
    invoke('load_scene', { id: sceneId })
      .then(() => {
        setSelectedScene(sceneId);
        console.log('Scene geladen:', sceneId);
      })
      .catch(console.error);
  };

  // Scene speichern
  const saveScene = async () => {
    if (!newSceneName.trim()) return;

    try {
      // Aktuellen Mixer-State von allen Komponenten sammeln
      const [strips, buses, fxChain, routing, master, voiceFx] = await Promise.all([
        invoke('get_strips').catch(() => []),
        invoke('get_buses').catch(() => []),
        invoke('get_fx_chain').catch(() => []),
        invoke('get_routing_matrix').catch(() => []),
        invoke('get_master').catch(() => ({})),
        invoke('get_voice_fx_state').catch(() => ({})),
      ]);

      // State als JSON serialisieren
      const stateJson = JSON.stringify({
        strips,
        buses,
        fx_chain: fxChain,
        routing,
        master,
        voice_fx: voiceFx,
        timestamp: Date.now(),
      });

      await invoke('save_scene', { name: newSceneName, stateJson });

      setShowSaveModal(false);
      setNewSceneName('');
      loadScenes();
    } catch (err) {
      console.error('Fehler beim Speichern der Scene:', err);
    }
  };

  return (
    <div className="flex items-center gap-2">
      {/* Scene-Dropdown */}
      <select
        className="bg-inox-strip border border-inox-subtle/20 rounded text-[6px] text-inox-text px-1 py-0.5 font-mono min-w-[80px]"
        value={selectedScene}
        onChange={(e) => loadScene(e.target.value)}
      >
        <option value="">Scene wählen...</option>
        {scenes.map((scene) => (
          <option key={scene.id} value={scene.id}>
            {scene.name}
          </option>
        ))}
      </select>

      {/* Save-Button */}
      <button
        onClick={() => setShowSaveModal(true)}
        className="text-[6px] font-bold uppercase tracking-[1px] px-2 py-0.5 rounded border border-inox-subtle/20 text-inox-muted hover:text-inox-dim hover:border-inox-subtle/40 transition-colors"
      >
        💾 SAVE
      </button>

      {/* Save Modal */}
      {showSaveModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-inox-panel border border-inox-subtle/20 rounded p-4 min-w-[300px]">
            <h3 className="text-[8px] font-bold text-inox-cyan tracking-wider uppercase mb-2">
              Scene speichern
            </h3>
            <input
              type="text"
              placeholder="Scene-Name..."
              value={newSceneName}
              onChange={(e) => setNewSceneName(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && saveScene()}
              className="w-full bg-inox-strip border border-inox-subtle/20 rounded text-[7px] text-inox-text px-2 py-1 mb-3 font-mono"
              autoFocus
            />
            <div className="flex gap-2">
              <button
                onClick={saveScene}
                className="flex-1 text-[6px] font-bold uppercase tracking-[1px] px-3 py-1 rounded bg-inox-cyan/40 text-inox-cyan hover:bg-inox-cyan/60 transition-colors"
              >
                Speichern
              </button>
              <button
                onClick={() => setShowSaveModal(false)}
                className="flex-1 text-[6px] font-bold uppercase tracking-[1px] px-3 py-1 rounded bg-inox-subtle text-inox-muted hover:bg-inox-strip transition-colors"
              >
                Abbrechen
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
