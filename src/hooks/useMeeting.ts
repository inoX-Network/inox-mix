// Hook: useMetering — VU/Peak/RMS Daten vom Backend empfangen

/** Hook für Echtzeit-Metering Daten (60fps vom Rust Backend) */
export function useMetering() {
  // TODO: Tauri Event-Listener für level_update
  // TODO: MeterData[] State
  // TODO: requestAnimationFrame für 60fps Updates

  return {
    // TODO: levels: MeterData[]
    // TODO: isConnected: boolean
  };
}

export default useMetering;
