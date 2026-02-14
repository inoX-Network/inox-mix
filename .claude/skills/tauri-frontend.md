# Skill: Tauri Frontend (React + TypeScript)

## Tauri IPC
```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Command aufrufen
const levels = await invoke<LevelData[]>('get_levels');

// Event empfangen (Echtzeit-Metering)
const unlisten = await listen<LevelData>('level_update', (event) => {
  updateVU(event.payload);
});
```

## State Management (Zustand)
```typescript
import { create } from 'zustand';

interface MixerStore {
  strips: InputStrip[];
  setVolume: (id: string, value: number) => void;
}

export const useMixerStore = create<MixerStore>((set) => ({
  strips: [],
  setVolume: (id, value) => {
    invoke('set_volume', { stripId: id, value });
    set((state) => ({
      strips: state.strips.map(s => s.id === id ? { ...s, volume: value } : s)
    }));
  },
}));
```

## Komponenten-Pattern
```typescript
interface StripProps {
  id: string;
  label: string;
  icon: string;
  color: 'cyan' | 'orange';
}

export default function Strip({ id, label, icon, color }: StripProps) {
  const { volume, setVolume } = useMixerStore(s => ({
    volume: s.strips.find(st => st.id === id)?.volume ?? 75,
    setVolume: s.setVolume,
  }));
  // ...
}
```

## Animation Pattern (VU-Meter)
```typescript
const rafRef = useRef<number>();
useEffect(() => {
  const animate = () => {
    // Read levels from store (atomic, kein IPC pro Frame)
    rafRef.current = requestAnimationFrame(animate);
  };
  rafRef.current = requestAnimationFrame(animate);
  return () => cancelAnimationFrame(rafRef.current!);
}, []);
```

## Tailwind Custom Config
```typescript
// tailwind.config.ts
export default {
  theme: {
    extend: {
      colors: {
        'inox-cyan': '#00e5ff',
        'inox-orange': '#ff8c00',
        'inox-red': '#ff1744',
        'inox-green': '#4caf50',
        'inox-amber': '#e6a117',
        'inox-bg': '#08090b',
        'inox-panel': '#0d0f13',
        'inox-strip': '#111318',
      },
      fontFamily: {
        oxanium: ['Oxanium', 'monospace'],
      },
    },
  },
};
```

## NPM Packages (Pflicht)
- react, react-dom (18.x)
- @tauri-apps/api (2.x)
- zustand (State)
- tailwindcss, postcss, autoprefixer
- typescript, @types/react
- vite, @vitejs/plugin-react
