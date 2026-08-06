import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  theme: {
    extend: {
      colors: {
        "levee-deep": "#0B2545",
        "levee-mid": "#13507A",
        "levee-surface": "#1B6CA8",
        "levee-light": "#A3D5FF",
        "levee-mark": "#E8F4FD",
        "levee-trigger": "#FF6B35",
      },
      fontFamily: {
        display: ["DM Serif Display", "Georgia", "serif"],
        body: ["Inter", "system-ui", "sans-serif"],
        tabular: ["JetBrains Mono", "Menlo", "monospace"],
      },
    },
  },
  plugins: [],
};

export default config;
