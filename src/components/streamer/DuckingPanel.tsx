// Komponente: DuckingPanel — Sidechain-Ducking Einstellungen
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import Slider from '../mixer/Slider';

/** Ducking-Panel mit Amount, Attack, Release und Threshold Reglern */
interface DuckingPanelProps {
  /** Ducking aktiviert */
  initialEnabled?: boolean;
  /** Initiale Amount (dB Reduktion) */
  initialAmount?: number;
  /** Initiale Attack (ms) */
  initialAttack?: number;
  /** Initiale Release (ms) */
  initialRelease?: number;
  /** Initialer Threshold (dB) */
  initialThreshold?: number;
}

function DuckingPanel({
  initialEnabled = false,
  initialAmount = -12,
  initialAttack = 50,
  initialRelease = 300,
  initialThreshold = -20,
}: DuckingPanelProps) {
  const [enabled, setEnabled] = useState(initialEnabled);
  const [amount, setAmount] = useState(initialAmount);
  const [attack, setAttack] = useState(initialAttack);
  const [release, setRelease] = useState(initialRelease);
  const [threshold, setThreshold] = useState(initialThreshold);

  const handleToggle = async () => {
    const newEnabled = !enabled;
    setEnabled(newEnabled);

    try {
      await invoke('set_ducking_enabled', { enabled: newEnabled });
    } catch (err) {
      console.error('Fehler beim Setzen des Ducking-Status:', err);
    }
  };

  const handleAmountChange = (value: number) => {
    // -30dB bis 0dB
    const dbValue = value * -30;
    setAmount(dbValue);
    // TODO: Invoke backend command
  };

  const handleAttackChange = (value: number) => {
    // 10ms bis 500ms
    const msValue = 10 + value * 490;
    setAttack(msValue);
    // TODO: Invoke backend command
  };

  const handleReleaseChange = (value: number) => {
    // 50ms bis 2000ms
    const msValue = 50 + value * 1950;
    setRelease(msValue);
    // TODO: Invoke backend command
  };

  const handleThresholdChange = (value: number) => {
    // -50dB bis 0dB
    const dbValue = value * -50;
    setThreshold(dbValue);
    // TODO: Invoke backend command
  };

  return (
    <div className="p-3 border-b border-[rgba(255,255,255,0.05)]">
      {/* Header mit Toggle */}
      <div className="mb-3 flex items-center justify-between">
        <div>
          <h3 className="text-[7px] font-extrabold uppercase tracking-wider text-cyan-500">
            Ducking
          </h3>
          <p className="text-[5px] text-gray-600 mt-0.5">
            Sidechain-Kompressor
          </p>
        </div>

        {/* Toggle Switch */}
        <button
          onClick={handleToggle}
          className={`relative inline-flex h-5 w-9 items-center rounded-full transition-colors ${
            enabled ? 'bg-cyan-500' : 'bg-gray-700'
          }`}
        >
          <span
            className={`inline-block h-3 w-3 transform rounded-full bg-white transition-transform ${
              enabled ? 'translate-x-5' : 'translate-x-1'
            }`}
          />
        </button>
      </div>

      {/* Parameter Sliders */}
      <div className="space-y-3 opacity-100 transition-opacity" style={{ opacity: enabled ? 1 : 0.4 }}>
        {/* Amount (dB Reduktion) */}
        <Slider
          label="AMOUNT"
          value={Math.abs(amount) / 30}
          onChange={handleAmountChange}
          color="#00e5ff"
          unit="dB"
          disabled={!enabled}
        />

        {/* Attack (ms) */}
        <Slider
          label="ATTACK"
          value={(attack - 10) / 490}
          onChange={handleAttackChange}
          color="#00e5ff"
          unit="ms"
          disabled={!enabled}
        />

        {/* Release (ms) */}
        <Slider
          label="RELEASE"
          value={(release - 50) / 1950}
          onChange={handleReleaseChange}
          color="#00e5ff"
          unit="ms"
          disabled={!enabled}
        />

        {/* Threshold (dB) */}
        <Slider
          label="THRESHOLD"
          value={Math.abs(threshold) / 50}
          onChange={handleThresholdChange}
          color="#00e5ff"
          unit="dB"
          disabled={!enabled}
        />
      </div>

      {/* Info Box */}
      {enabled && (
        <div className="mt-3 p-2 bg-cyan-500/10 border border-cyan-500/30 rounded text-center">
          <p className="text-[5px] text-cyan-400">
            Musik wird um <strong>{Math.abs(amount).toFixed(0)} dB</strong> reduziert wenn Mikrofon über <strong>{Math.abs(threshold).toFixed(0)} dB</strong>
          </p>
        </div>
      )}
    </div>
  );
}

export default DuckingPanel;
