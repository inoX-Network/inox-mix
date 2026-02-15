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
    <div className="flex items-center gap-3">
      {/* Scene-Dropdown (Custom Dark Style) */}
      <select
        className="bg-inox-strip border rounded text-[11px] px-3 py-1 font-oxanium min-w-[120px] transition-colors cursor-pointer appearance-none pr-7"
        style={{
          borderColor: 'rgba(255,255,255,0.08)',
          borderRadius: '6px',
          color: 'rgba(255,255,255,0.5)',
          backgroundImage: `url("data:image/svg+xml,%3Csvg width='15' height='9' viewBox='0 0 15 9' fill='none' xmlns='http://www.w3.org/2000/svg'%3E%3Cpath d='M1.5 1.5L7.5 7.5L13.5 1.5' stroke='rgba(255,255,255,0.2)' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E")`,
          backgroundRepeat: 'no-repeat',
          backgroundPosition: 'right 6px center',
        }}
        value={selectedScene}
        onChange={(e) => loadScene(e.target.value)}
        onMouseEnter={(e) => e.currentTarget.style.borderColor = 'rgba(0,229,255,0.15)'}
        onMouseLeave={(e) => e.currentTarget.style.borderColor = 'rgba(255,255,255,0.08)'}
        onFocus={(e) => e.currentTarget.style.borderColor = 'rgba(0,229,255,0.25)'}
        onBlur={(e) => e.currentTarget.style.borderColor = 'rgba(255,255,255,0.08)'}
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
        className="text-[9px] font-bold uppercase tracking-[1.5px] px-3 py-1 rounded border border-inox-subtle/20 text-inox-muted hover:text-inox-dim hover:border-inox-subtle/40 transition-colors"
      >
        💾 SAVE
      </button>

      {/* Save Modal */}
      {showSaveModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-inox-panel border border-inox-subtle/20 rounded p-6 min-w-[450px]">
            <h3 className="text-[12px] font-bold text-inox-cyan tracking-wider uppercase mb-3">
              Scene speichern
            </h3>
            <input
              type="text"
              placeholder="Scene-Name..."
              value={newSceneName}
              onChange={(e) => setNewSceneName(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && saveScene()}
              className="w-full bg-inox-strip border border-inox-subtle/20 rounded text-[11px] text-inox-text px-3 py-2 mb-5 font-mono"
              autoFocus
            />
            <div className="flex gap-3">
              <button
                onClick={saveScene}
                className="flex-1 text-[9px] font-bold uppercase tracking-[1.5px] px-5 py-2 rounded bg-inox-cyan/40 text-inox-cyan hover:bg-inox-cyan/60 transition-colors"
              >
                Speichern
              </button>
              <button
                onClick={() => setShowSaveModal(false)}
                className="flex-1 text-[9px] font-bold uppercase tracking-[1.5px] px-5 py-2 rounded bg-inox-subtle text-inox-muted hover:bg-inox-strip transition-colors"
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
