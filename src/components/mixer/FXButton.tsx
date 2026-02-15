// Component: FXButton — FX-Chain Ein/Aus Toggle pro Strip

interface FXButtonProps {
  /** FX-Chain aktiv */
  active: boolean;
  /** Callback bei Klick */
  onClick: () => void;
}

/** FX-Chain Toggle-Button — Spec: 5.5px font, full width */
export default function FXButton({ active, onClick }: FXButtonProps) {
  return (
    <button
      className="w-full text-center cursor-pointer transition-all"
      style={{
        padding: '2px 5px',
        fontSize: '5.5px',
        fontWeight: 700,
        letterSpacing: '0.7px',
        textTransform: 'uppercase' as const,
        borderRadius: '2px',
        border: `1px solid ${active ? '#00e5ff' : 'rgba(255,255,255,0.05)'}`,
        background: active ? 'rgba(0,229,255,0.04)' : 'rgba(255,255,255,0.01)',
        color: active ? '#00e5ff' : 'rgba(255,255,255,0.15)',
        boxShadow: active ? '0 0 5px rgba(0,229,255,0.08)' : 'none',
      }}
      onClick={onClick}
      aria-label="FX-Chain"
      aria-pressed={active}
    >
      FX {active ? '●' : '○'}
    </button>
  );
}
