/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        atom: {
          base: "#101114",
          panel: "#17181d",
          elevated: "#1e2026",
          line: "#2b2d35",
          muted: "#858894",
          text: "#f3f4f6",
          accent: "#b8f36b",
        },
      },
    },
  },
  plugins: [],
};
