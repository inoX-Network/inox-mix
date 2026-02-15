// Component: BusButton — Bus-Routing Toggle (A1, A2, B1, B2)

interface BusButtonProps {
  /** Bus-Name (z.B. "A1", "B1") */
  busId: string;
  /** Aktiv/Inaktiv */
  active: boolean;
  /** Callback bei Klick */
  onClick: () => void;
}

/** Mini Bus-Routing Button — Spec: 15×11px, 4.5px font, 800 weight */
export default function BusButton({ busId, active, onClick }: BusButtonProps) {
  const isABus = busId.startsWith('A');
  const activeColor = isABus ? '#00e5ff' : '#ff8c00';

  return (
    <button
      className="inline-flex items-center justify-center rounded-sm transition-all"
      style={{
        width: '15px',
        height: '11px',
        fontSize: '4.5px',
        fontWeight: 800,
        border: `1px solid ${active ? 'transparent' : 'rgba(255,255,255,0.05)'}`,
        background: active ? activeColor : 'rgba(255,255,255,0.01)',
        color: active ? '#000' : 'rgba(255,255,255,0.13)',
      }}
      onClick={onClick}
      aria-label={`Bus ${busId}`}
      aria-pressed={active}
    >
      {busId}
    </button>
  );
}
