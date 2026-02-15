// Hook: useMetering — Echtzeit-VU-Meter Daten aus Tauri Events

import { useState, useEffect } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

/** Stereo-Messwerte für einen Strip */
interface StripLevels {
  strip_id: string;
  peak_l: number;  // dB
  peak_r: number;  // dB
  rms_l: number;   // dB
  rms_r: number;   // dB
  clipping: boolean;
}

/** Metering-Daten für alle Strips */
interface MeteringData {
  [stripId: string]: StripLevels;
}

/**
 * Hook für Echtzeit-Metering-Daten
 *
 * Empfängt "metering-update" Events vom Backend (60fps)
 * und stellt die aktuellen Peak/RMS Werte zur Verfügung.
 *
 * @returns Metering-Daten für alle Strips
 *
 * @example
 * const metering = useMetering();
 * const stripLevels = metering['hw-mic-1'];
 * if (stripLevels) {
 *   console.log('Peak L:', stripLevels.peak_l, 'dB');
 * }
 */
export function useMetering(): MeteringData {
  const [metering, setMetering] = useState<MeteringData>({});

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;

    // Tauri Event Listener registrieren
    listen<StripLevels[]>('metering-update', (event) => {
      const levels = event.payload;

      // Array in Object umwandeln (Key: strip_id)
      const meteringMap: MeteringData = {};
      for (const level of levels) {
        meteringMap[level.strip_id] = level;
      }

      setMetering(meteringMap);
    }).then((fn) => {
      unlisten = fn;
    }).catch((err) => {
      console.error('Fehler beim Registrieren des Metering-Listeners:', err);
    });

    // Cleanup beim Unmount
    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return metering;
}

/**
 * Hook für Metering-Daten eines einzelnen Strips
 *
 * @param stripId - Die Strip-ID (z.B. "hw-mic-1")
 * @returns Metering-Daten für den Strip oder null
 *
 * @example
 * const levels = useStripMetering('hw-mic-1');
 * if (levels) {
 *   <VUMeter peak={levels.peak_l} rms={levels.rms_l} />
 * }
 */
export function useStripMetering(stripId: string): StripLevels | null {
  const metering = useMetering();
  return metering[stripId] || null;
}
