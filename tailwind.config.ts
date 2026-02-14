// Tailwind-Konfiguration für inoX-MIX — Farben aus DESIGN-SYSTEM.md
import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Primärfarben
        "inox-cyan": "#00e5ff",
        "inox-orange": "#ff8c00",
        // Funktionale Farben
        "inox-red": "#ff1744",
        "inox-green": "#4caf50",
        "inox-amber": "#e6a117",
        // Hintergründe
        "inox-bg": "#08090b",
        "inox-panel": "#0d0f13",
        "inox-strip": "#111318",
        // Text-Abstufungen
        "inox-text": "#cccccc",
        "inox-dim": "#888888",
        "inox-muted": "#666666",
        "inox-subtle": "#444444",
        "inox-faint": "#555555",
      },
      fontFamily: {
        oxanium: ["Oxanium", "monospace"],
      },
    },
  },
  plugins: [],
} satisfies Config;
