// Tailwind-Konfiguration für inoX-MIX — Farben aus DESIGN-SYSTEM.md
import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        "inox-cyan": "#00e5ff",
        "inox-orange": "#ff8c00",
        "inox-red": "#ff1744",
        "inox-green": "#4caf50",
        "inox-amber": "#e6a117",
        "inox-bg": "#08090b",
        "inox-panel": "#0d0f13",
        "inox-strip": "#111318",
      },
      fontFamily: {
        oxanium: ["Oxanium", "monospace"],
      },
    },
  },
  plugins: [],
} satisfies Config;
