// Komponente: SettingsPage — Einstellungen (Audio, Geräte, UI, Updates)

import { useState } from 'react';
import UpdateSection from './UpdateSection';

/** Einstellungen-Kategorien */
type SettingsCategory = 'audio' | 'devices' | 'ui' | 'system';

/** Einstellungen-Seite mit Kategorien für Audio, Geräte, UI und System */
interface SettingsPageProps {}

function SettingsPage(_props: SettingsPageProps) {
  const [activeCategory, setActiveCategory] = useState<SettingsCategory>('system');

  const categories: { id: SettingsCategory; label: string; icon: string }[] = [
    { id: 'audio', label: 'Audio', icon: '🎵' },
    { id: 'devices', label: 'Geräte', icon: '🎤' },
    { id: 'ui', label: 'Oberfläche', icon: '🎨' },
    { id: 'system', label: 'System', icon: '⚙️' },
  ];

  return (
    <div className="flex h-full bg-background">
      {/* Sidebar: Kategorie-Navigation */}
      <div className="w-56 bg-panel border-r border-gray-800 p-4">
        <h2 className="text-xl font-bold text-gray-100 mb-6">Einstellungen</h2>
        <nav className="space-y-2">
          {categories.map((cat) => (
            <button
              key={cat.id}
              onClick={() => setActiveCategory(cat.id)}
              className={`w-full flex items-center gap-3 px-4 py-3 rounded-lg text-left transition-colors ${
                activeCategory === cat.id
                  ? 'bg-primary/20 text-primary border border-primary/30'
                  : 'text-gray-400 hover:bg-gray-800 hover:text-gray-200'
              }`}
            >
              <span className="text-xl">{cat.icon}</span>
              <span className="font-medium">{cat.label}</span>
            </button>
          ))}
        </nav>
      </div>

      {/* Content: Einstellungen-Bereich */}
      <div className="flex-1 p-8 overflow-y-auto">
        {activeCategory === 'audio' && (
          <div className="max-w-2xl">
            <h3 className="text-lg font-semibold text-gray-100 mb-4">Audio-Einstellungen</h3>
            <p className="text-sm text-gray-400">TODO: Sample Rate, Buffer Size, Latenz</p>
          </div>
        )}

        {activeCategory === 'devices' && (
          <div className="max-w-2xl">
            <h3 className="text-lg font-semibold text-gray-100 mb-4">Geräte-Einstellungen</h3>
            <p className="text-sm text-gray-400">TODO: Input/Output Zuordnung, Geräte-Auswahl</p>
          </div>
        )}

        {activeCategory === 'ui' && (
          <div className="max-w-2xl">
            <h3 className="text-lg font-semibold text-gray-100 mb-4">Oberfläche</h3>
            <p className="text-sm text-gray-400">TODO: Theme, Sprache, Layout</p>
          </div>
        )}

        {activeCategory === 'system' && (
          <div className="max-w-2xl">
            <UpdateSection />
            {/* TODO: Reset, Export/Import, Logs */}
          </div>
        )}
      </div>
    </div>
  );
}

export default SettingsPage;
