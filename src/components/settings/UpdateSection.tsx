// Komponente: UpdateSection — Update-Einstellungen und Update-Check

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

/** Update-Information vom Backend */
interface UpdateInfo {
  version: string;
  notes: string;
  url: string;
  date: string;
  available: boolean;
}

/** Update-Sektion in den Einstellungen */
function UpdateSection() {
  const [checking, setChecking] = useState(false);
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [installing, setInstalling] = useState(false);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  // Auto-Check Setting
  const [autoCheck, setAutoCheck] = useState(true);

  useEffect(() => {
    // Auto-Check Setting laden
    invoke<string | null>('get_config', { key: 'auto_update_check' })
      .then((value) => {
        if (value !== null) {
          setAutoCheck(value === 'true');
        }
      })
      .catch((err) => console.error('Auto-Check Setting laden fehlgeschlagen:', err));

    // Event-Listener für Update-Fortschritt
    const unlisten1 = listen<number>('update-progress', (event) => {
      setProgress(event.payload);
    });

    const unlisten2 = listen('update-installed', () => {
      setInstalling(false);
      setProgress(100);
    });

    return () => {
      unlisten1.then((fn) => fn());
      unlisten2.then((fn) => fn());
    };
  }, []);

  /** Auf Updates prüfen */
  const checkForUpdates = async () => {
    setChecking(true);
    setError(null);
    setUpdateInfo(null);

    try {
      const info = await invoke<UpdateInfo | null>('check_for_updates');
      if (info) {
        setUpdateInfo(info);
      } else {
        // Keine Updates verfügbar
        setUpdateInfo({
          version: '',
          notes: '',
          url: '',
          date: '',
          available: false,
        });
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setChecking(false);
    }
  };

  /** Update installieren */
  const installUpdate = async () => {
    if (!updateInfo || !updateInfo.available) return;

    setInstalling(true);
    setError(null);
    setProgress(0);

    try {
      await invoke('install_update');
    } catch (err) {
      setError(String(err));
      setInstalling(false);
    }
  };

  /** Auto-Check Setting ändern */
  const toggleAutoCheck = async (enabled: boolean) => {
    setAutoCheck(enabled);
    try {
      await invoke('set_config', {
        key: 'auto_update_check',
        value: enabled ? 'true' : 'false',
      });
    } catch (err) {
      console.error('Auto-Check Setting speichern fehlgeschlagen:', err);
    }
  };

  return (
    <div className="space-y-6">
      {/* Überschrift */}
      <div>
        <h3 className="text-lg font-semibold text-gray-100">Updates</h3>
        <p className="text-sm text-gray-400">Automatische Updates und Versionsprüfung</p>
      </div>

      {/* Auto-Check Toggle */}
      <div className="flex items-center justify-between p-4 bg-panel rounded-lg border border-gray-800">
        <div>
          <p className="text-sm font-medium text-gray-100">Automatische Update-Prüfung</p>
          <p className="text-xs text-gray-500">Beim Start und alle 24 Stunden</p>
        </div>
        <button
          onClick={() => toggleAutoCheck(!autoCheck)}
          className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
            autoCheck ? 'bg-primary' : 'bg-gray-700'
          }`}
        >
          <span
            className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
              autoCheck ? 'translate-x-6' : 'translate-x-1'
            }`}
          />
        </button>
      </div>

      {/* Update-Check Button */}
      <button
        onClick={checkForUpdates}
        disabled={checking}
        className="w-full px-4 py-3 bg-primary/10 hover:bg-primary/20 disabled:bg-gray-800 disabled:text-gray-500 text-primary font-medium rounded-lg border border-primary/30 transition-colors"
      >
        {checking ? 'Prüfe auf Updates...' : 'Jetzt auf Updates prüfen'}
      </button>

      {/* Fehler */}
      {error && (
        <div className="p-4 bg-error/10 border border-error/30 rounded-lg">
          <p className="text-sm text-error">{error}</p>
        </div>
      )}

      {/* Update verfügbar */}
      {updateInfo && updateInfo.available && (
        <div className="p-4 bg-primary/10 border border-primary/30 rounded-lg space-y-3">
          <div>
            <h4 className="text-base font-semibold text-primary">
              Neue Version verfügbar: v{updateInfo.version}
            </h4>
            <p className="text-xs text-gray-400 mt-1">{updateInfo.date}</p>
          </div>

          {updateInfo.notes && (
            <div className="p-3 bg-background rounded-md">
              <p className="text-sm text-gray-300 whitespace-pre-wrap">{updateInfo.notes}</p>
            </div>
          )}

          {installing ? (
            <div className="space-y-2">
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-400">Installiere Update...</span>
                <span className="text-primary font-medium">{Math.round(progress)}%</span>
              </div>
              <div className="w-full h-2 bg-gray-800 rounded-full overflow-hidden">
                <div
                  className="h-full bg-primary transition-all duration-300"
                  style={{ width: `${progress}%` }}
                />
              </div>
            </div>
          ) : (
            <button
              onClick={installUpdate}
              className="w-full px-4 py-2 bg-primary hover:bg-primary-hover text-background font-medium rounded-lg transition-colors"
            >
              Jetzt installieren
            </button>
          )}
        </div>
      )}

      {/* Keine Updates */}
      {updateInfo && !updateInfo.available && (
        <div className="p-4 bg-success/10 border border-success/30 rounded-lg">
          <p className="text-sm text-success">Sie verwenden die neueste Version</p>
        </div>
      )}
    </div>
  );
}

export default UpdateSection;
