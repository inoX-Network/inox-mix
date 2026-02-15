// Komponente: SettingsPage — Einstellungen (Audio, Geräte, Hotkeys, Aufnahme, UI, System)

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import UpdateSection from './UpdateSection';

/** Einstellungen-Kategorien */
type SettingsCategory = 'audio' | 'recording' | 'bleeper' | 'ui' | 'system';

/** Audio-Gerät aus PipeWire */
interface AudioDevice {
  id: string;
  name: string;
  type: 'input' | 'output';
}

/** Einstellungen-Seite mit Kategorien */
interface SettingsPageProps {}

function SettingsPage(_props: SettingsPageProps) {
  const [activeCategory, setActiveCategory] = useState<SettingsCategory>('audio');

  // Audio Settings
  const [sampleRate, setSampleRate] = useState(48000);
  const [bufferSize, setBufferSize] = useState(256);
  const [bitDepth, setBitDepth] = useState('32-float');
  const [audioDevices, setAudioDevices] = useState<AudioDevice[]>([]);

  // Recording Settings
  const [recordingPath, setRecordingPath] = useState('~/Recordings');
  const [recordingFormat, setRecordingFormat] = useState<'FLAC' | 'WAV'>('FLAC');
  const [autoRecord, setAutoRecord] = useState(false);

  // Bleeper Settings
  const [bleeperWords, setBleeperWords] = useState<string[]>(['Scheiße', 'Fuck', 'Arsch']);
  const [newWord, setNewWord] = useState('');

  // UI Settings
  const [language, setLanguage] = useState<'DE' | 'EN'>('DE');
  const [theme, setTheme] = useState<'Dark' | 'Light' | 'System'>('Dark');
  const [layout, setLayout] = useState<'Standard' | 'Erweitert' | 'Kompakt'>('Standard');

  // System Settings
  const [autoStart, setAutoStart] = useState(false);
  const [trayIcon, setTrayIcon] = useState(true);
  const [autoUpdate, setAutoUpdate] = useState(true);

  const categories: { id: SettingsCategory; label: string; icon: string }[] = [
    { id: 'audio', label: 'Audio', icon: '🎵' },
    { id: 'recording', label: 'Aufnahme', icon: '🎙️' },
    { id: 'bleeper', label: 'Bleeper', icon: '🔇' },
    { id: 'ui', label: 'Oberfläche', icon: '🎨' },
    { id: 'system', label: 'System', icon: '⚙️' },
  ];

  // Load audio devices on mount
  useEffect(() => {
    loadAudioDevices();
  }, []);

  const loadAudioDevices = async () => {
    try {
      // TODO: Backend command implementieren
      const mockDevices: AudioDevice[] = [
        { id: 'hw:0,0', name: 'Built-in Audio', type: 'input' },
        { id: 'hw:1,0', name: 'USB Microphone', type: 'input' },
        { id: 'hw:0,1', name: 'Built-in Speakers', type: 'output' },
      ];
      setAudioDevices(mockDevices);
    } catch (err) {
      console.error('Fehler beim Laden der Audio-Geräte:', err);
    }
  };

  const handleSampleRateChange = async (rate: number) => {
    setSampleRate(rate);
    try {
      await invoke('set_config', { key: 'audio.sample_rate', value: String(rate) });
    } catch (err) {
      console.error('Fehler beim Setzen der Sample-Rate:', err);
    }
  };

  const handleBufferSizeChange = async (size: number) => {
    setBufferSize(size);
    try {
      await invoke('set_config', { key: 'audio.buffer_size', value: String(size) });
    } catch (err) {
      console.error('Fehler beim Setzen der Buffer-Size:', err);
    }
  };

  const handleExportConfig = async () => {
    try {
      const configJson = await invoke<string>('export_config');
      // Download as JSON file
      const blob = new Blob([configJson], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `inox-mix-config-${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);
    } catch (err) {
      console.error('Fehler beim Exportieren der Config:', err);
    }
  };

  const handleAddBleeperWord = () => {
    if (newWord.trim() && !bleeperWords.includes(newWord.trim())) {
      setBleeperWords([...bleeperWords, newWord.trim()]);
      setNewWord('');
      // TODO: Backend sync
    }
  };

  const handleRemoveBleeperWord = (word: string) => {
    setBleeperWords(bleeperWords.filter((w) => w !== word));
    // TODO: Backend sync
  };

  return (
    <div className="flex h-full bg-background">
      {/* Sidebar: Kategorie-Navigation */}
      <div className="w-56 bg-inox-panel border-r border-inox-subtle/20 p-4">
        <h2 className="text-[10px] font-extrabold uppercase tracking-wider text-inox-cyan mb-6">
          Einstellungen
        </h2>
        <nav className="space-y-2">
          {categories.map((cat) => (
            <button
              key={cat.id}
              onClick={() => setActiveCategory(cat.id)}
              className={`w-full flex items-center gap-3 px-3 py-2 rounded text-left transition-colors ${
                activeCategory === cat.id
                  ? 'bg-inox-cyan/20 text-inox-cyan border border-inox-cyan/30'
                  : 'text-inox-muted hover:bg-inox-strip hover:text-inox-dim'
              }`}
            >
              <span className="text-[12px]">{cat.icon}</span>
              <span className="text-[6px] font-bold uppercase tracking-wide">{cat.label}</span>
            </button>
          ))}
        </nav>
      </div>

      {/* Content: Einstellungen-Bereich */}
      <div className="flex-1 p-6 overflow-y-auto">
        {/* Audio Settings */}
        {activeCategory === 'audio' && (
          <div className="max-w-2xl space-y-6">
            <div>
              <h3 className="text-[8px] font-extrabold uppercase tracking-wider text-inox-cyan mb-4">
                Audio-Einstellungen
              </h3>
              <p className="text-[5px] text-inox-muted mb-4">
                Sample-Rate, Buffer-Size und Bit-Depth konfigurieren
              </p>
            </div>

            {/* Sample Rate */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Sample Rate
              </label>
              <div className="flex gap-2">
                {[44100, 48000, 96000].map((rate) => (
                  <button
                    key={rate}
                    onClick={() => handleSampleRateChange(rate)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      sampleRate === rate
                        ? 'bg-inox-cyan text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {rate / 1000} kHz
                  </button>
                ))}
              </div>
            </div>

            {/* Buffer Size */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Buffer Size
              </label>
              <div className="flex gap-2">
                {[64, 128, 256, 512, 1024].map((size) => (
                  <button
                    key={size}
                    onClick={() => handleBufferSizeChange(size)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      bufferSize === size
                        ? 'bg-inox-cyan text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {size}
                  </button>
                ))}
              </div>
              <p className="mt-2 text-[4.5px] text-inox-muted">
                Geschätzte Latenz: ~{((bufferSize / sampleRate) * 1000).toFixed(1)} ms
              </p>
            </div>

            {/* Bit Depth */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Bit Depth
              </label>
              <div className="flex gap-2">
                {['16', '24', '32-float'].map((depth) => (
                  <button
                    key={depth}
                    onClick={() => setBitDepth(depth)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      bitDepth === depth
                        ? 'bg-inox-cyan text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {depth} bit
                  </button>
                ))}
              </div>
            </div>

            {/* Audio Devices */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-3">
                Audio-Geräte
              </label>
              <div className="space-y-2">
                {audioDevices.map((device) => (
                  <div
                    key={device.id}
                    className="flex items-center justify-between px-3 py-2 bg-inox-panel rounded"
                  >
                    <div className="flex items-center gap-2">
                      <span className="text-[8px]">{device.type === 'input' ? '🎤' : '🔊'}</span>
                      <span className="text-[5px] text-inox-dim">{device.name}</span>
                    </div>
                    <span className="text-[4.5px] text-inox-muted font-mono">{device.id}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Recording Settings */}
        {activeCategory === 'recording' && (
          <div className="max-w-2xl space-y-6">
            <div>
              <h3 className="text-[8px] font-extrabold uppercase tracking-wider text-inox-cyan mb-4">
                Aufnahme-Einstellungen
              </h3>
            </div>

            {/* Recording Path */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Speicherort
              </label>
              <input
                type="text"
                value={recordingPath}
                onChange={(e) => setRecordingPath(e.target.value)}
                className="w-full px-3 py-2 bg-inox-panel border border-inox-subtle/20 rounded text-[5px] text-inox-text font-mono"
              />
            </div>

            {/* Format */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Format
              </label>
              <div className="flex gap-2">
                {(['FLAC', 'WAV'] as const).map((format) => (
                  <button
                    key={format}
                    onClick={() => setRecordingFormat(format)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      recordingFormat === format
                        ? 'bg-inox-orange text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {format}
                  </button>
                ))}
              </div>
            </div>

            {/* Auto-Record */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded flex items-center justify-between">
              <div>
                <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim">
                  Auto-Aufnahme bei Stream-Start
                </label>
                <p className="text-[4.5px] text-inox-muted mt-0.5">
                  Startet automatisch Aufnahme wenn Stream beginnt
                </p>
              </div>
              <button
                onClick={() => setAutoRecord(!autoRecord)}
                className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${
                  autoRecord ? 'bg-inox-orange' : 'bg-inox-subtle'
                }`}
              >
                <span
                  className={`inline-block h-3 w-3 transform rounded-full bg-white transition-transform ${
                    autoRecord ? 'translate-x-5' : 'translate-x-1'
                  }`}
                />
              </button>
            </div>
          </div>
        )}

        {/* Bleeper Settings */}
        {activeCategory === 'bleeper' && (
          <div className="max-w-2xl space-y-6">
            <div>
              <h3 className="text-[8px] font-extrabold uppercase tracking-wider text-inox-error mb-4">
                Bleeper-Wortliste
              </h3>
              <p className="text-[5px] text-inox-muted mb-4">
                Verwalte die Liste der zu zensierenden Wörter
              </p>
            </div>

            {/* Add Word */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Wort hinzufügen
              </label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={newWord}
                  onChange={(e) => setNewWord(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleAddBleeperWord()}
                  placeholder="Neues Wort..."
                  className="flex-1 px-3 py-2 bg-inox-panel border border-inox-subtle/20 rounded text-[5px] text-inox-text"
                />
                <button
                  onClick={handleAddBleeperWord}
                  className="px-4 py-2 bg-inox-error text-white text-[5px] font-bold uppercase rounded hover:bg-inox-error/80 transition-colors"
                >
                  Hinzufügen
                </button>
              </div>
            </div>

            {/* Word List */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-3">
                Aktive Wörter ({bleeperWords.length})
              </label>
              <div className="flex flex-wrap gap-2">
                {bleeperWords.map((word) => (
                  <div
                    key={word}
                    className="flex items-center gap-2 px-3 py-1.5 bg-inox-error/20 border border-inox-error/30 rounded"
                  >
                    <span className="text-[5px] text-inox-error font-medium">{word}</span>
                    <button
                      onClick={() => handleRemoveBleeperWord(word)}
                      className="text-[8px] text-inox-error hover:text-white transition-colors"
                    >
                      ×
                    </button>
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* UI Settings */}
        {activeCategory === 'ui' && (
          <div className="max-w-2xl space-y-6">
            <div>
              <h3 className="text-[8px] font-extrabold uppercase tracking-wider text-inox-cyan mb-4">
                Oberfläche
              </h3>
            </div>

            {/* Language */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Sprache
              </label>
              <div className="flex gap-2">
                {(['DE', 'EN'] as const).map((lang) => (
                  <button
                    key={lang}
                    onClick={() => setLanguage(lang)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      language === lang
                        ? 'bg-inox-cyan text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {lang === 'DE' ? '🇩🇪 Deutsch' : '🇬🇧 English'}
                  </button>
                ))}
              </div>
            </div>

            {/* Theme */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Theme
              </label>
              <div className="flex gap-2">
                {(['Dark', 'Light', 'System'] as const).map((t) => (
                  <button
                    key={t}
                    onClick={() => setTheme(t)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      theme === t
                        ? 'bg-inox-cyan text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {t}
                  </button>
                ))}
              </div>
            </div>

            {/* Layout */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-2">
                Layout
              </label>
              <div className="flex gap-2">
                {(['Standard', 'Erweitert', 'Kompakt'] as const).map((l) => (
                  <button
                    key={l}
                    onClick={() => setLayout(l)}
                    className={`px-3 py-1.5 text-[5px] font-bold uppercase tracking-wide rounded transition-colors ${
                      layout === l
                        ? 'bg-inox-cyan text-background'
                        : 'bg-inox-panel text-inox-muted hover:bg-inox-subtle'
                    }`}
                  >
                    {l}
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* System Settings */}
        {activeCategory === 'system' && (
          <div className="max-w-2xl space-y-6">
            {/* Update Section (already implemented) */}
            <UpdateSection />

            {/* System Toggles */}
            <div className="space-y-4">
              {/* Auto-Start */}
              <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded flex items-center justify-between">
                <div>
                  <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim">
                    Mit System starten
                  </label>
                  <p className="text-[4.5px] text-inox-muted mt-0.5">
                    inoX-MIX automatisch beim Systemstart öffnen
                  </p>
                </div>
                <button
                  onClick={() => setAutoStart(!autoStart)}
                  className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${
                    autoStart ? 'bg-inox-cyan' : 'bg-inox-subtle'
                  }`}
                >
                  <span
                    className={`inline-block h-3 w-3 transform rounded-full bg-white transition-transform ${
                      autoStart ? 'translate-x-5' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>

              {/* Tray Icon */}
              <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded flex items-center justify-between">
                <div>
                  <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim">
                    Tray-Icon anzeigen
                  </label>
                  <p className="text-[4.5px] text-inox-muted mt-0.5">
                    Icon in der System-Leiste anzeigen
                  </p>
                </div>
                <button
                  onClick={() => setTrayIcon(!trayIcon)}
                  className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${
                    trayIcon ? 'bg-inox-cyan' : 'bg-inox-subtle'
                  }`}
                >
                  <span
                    className={`inline-block h-3 w-3 transform rounded-full bg-white transition-transform ${
                      trayIcon ? 'translate-x-5' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>

              {/* Auto-Update */}
              <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded flex items-center justify-between">
                <div>
                  <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim">
                    Automatische Updates
                  </label>
                  <p className="text-[4.5px] text-inox-muted mt-0.5">
                    Automatisch nach Updates suchen
                  </p>
                </div>
                <button
                  onClick={() => setAutoUpdate(!autoUpdate)}
                  className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${
                    autoUpdate ? 'bg-inox-cyan' : 'bg-inox-subtle'
                  }`}
                >
                  <span
                    className={`inline-block h-3 w-3 transform rounded-full bg-white transition-transform ${
                      autoUpdate ? 'translate-x-5' : 'translate-x-1'
                    }`}
                  />
                </button>
              </div>
            </div>

            {/* Export/Import */}
            <div className="p-4 bg-inox-strip border border-inox-subtle/20 rounded">
              <label className="block text-[6px] font-bold uppercase tracking-wide text-inox-dim mb-3">
                Konfiguration
              </label>
              <div className="flex gap-2">
                <button
                  onClick={handleExportConfig}
                  className="flex-1 px-4 py-2 bg-inox-cyan text-background text-[5px] font-bold uppercase rounded hover:bg-inox-cyan/80 transition-colors"
                >
                  💾 Exportieren
                </button>
                <button
                  className="flex-1 px-4 py-2 bg-inox-panel text-inox-muted text-[5px] font-bold uppercase rounded hover:bg-inox-subtle transition-colors"
                >
                  📥 Importieren
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default SettingsPage;
