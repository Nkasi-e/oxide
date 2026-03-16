/** @type {import('tailwindcss').Config} */
const config = {
  darkMode: ["class"],
  content: ["./pages/**/*.{ts,tsx}", "./components/**/*.{ts,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-outfit)", "system-ui", "sans-serif"],
        mono: ["var(--font-jetbrains)", "ui-monospace", "monospace"],
      },
      colors: {
        background: "#06060a",
        surface: "#0c0c12",
        "surface-hover": "#12121a",
        border: "#1e1e2e",
        "border-muted": "#16161d",
        muted: "#71717a",
        accent: "#f59e0b",
        "accent-dim": "#b45309",
        "accent-glow": "rgba(245, 158, 11, 0.25)",
        link: "#22d3ee",
        "link-hover": "#67e8f9",
        danger: "#f87171",
      },
      backgroundImage: {
        "grid-pattern":
          "linear-gradient(rgba(255,255,255,.02) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.02) 1px, transparent 1px)",
        "radial-glow":
          "radial-gradient(ellipse 80% 50% at 50% -20%, rgba(245, 158, 11, 0.12), transparent)",
      },
      boxShadow: {
        glow: "0 0 60px -15px rgba(245, 158, 11, 0.2)",
        "glow-sm": "0 0 24px -6px rgba(245, 158, 11, 0.15)",
        card: "0 4px 24px -4px rgba(0, 0, 0, 0.5), 0 0 0 1px rgba(255,255,255,0.04)",
        "card-hover":
          "0 8px 32px -8px rgba(0, 0, 0, 0.6), 0 0 0 1px rgba(245, 158, 11, 0.15)",
      },
      animation: {
        "fade-in": "fadeIn 0.4s ease-out",
        "fade-in-up": "fadeInUp 0.5s ease-out",
      },
      keyframes: {
        fadeIn: { "0%": { opacity: "0" }, "100%": { opacity: "1" } },
        fadeInUp: {
          "0%": { opacity: "0", transform: "translateY(12px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
      },
    },
  },
  plugins: [],
};

export default config;
