// Component: BusButton — Bus-Routing Toggle (A1, A2, B1, B2)

interface BusButtonProps {
  /** Bus-Name (z.B. "A1", "B1") */
  busId: string;
  /** Aktiv/Inaktiv */
  active: boolean;
  /** Callback bei Klick */
  onClick: () => void;
}

/**
 * Mini Bus-Routing Button
 * Cyan für A-Busse (Hardware), Orange für B-Busse (Virtual)
 */
export default function BusButton({ busId, active, onClick }: BusButtonProps) {
  const isABus = busId.startsWith('A');
  const activeColor = isABus ? 'bg-inox-cyan text-inox-bg' : 'bg-inox-orange text-inox-bg';
  const inactiveColor = isABus
    ? 'border-inox-cyan/40 text-inox-cyan/60'
    : 'border-inox-orange/40 text-inox-orange/60';

  return (
    <button
      className={`px-1 py-[1.5px] text-[5px] font-bold uppercase tracking-[0.4px] border rounded-sm transition-all ${
        active ? activeColor : `${inactiveColor} bg-transparent`
      }`}
      onClick={onClick}
      aria-label={`Bus ${busId}`}
      aria-pressed={active}
    >
      {busId}
    </button>
  );
}
