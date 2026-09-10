/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        neuro: {
          cyan: "#00f0ff",
          purple: "#9d4edd",
          dark: "#0a0c10",
          card: "#12151c",
          border: "#1e2433",
          accent: "#22d3ee"
        }
      }
    },
  },
  plugins: [],
}
