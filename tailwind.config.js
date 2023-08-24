/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {}
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
  daisyui: {
    themes: [
      "light",
      "dark",
      "bumblebee",
      "cupcake",
      "night",
      "business",
      "corporate",
      "dracula",
      "lemonade",
      "winter",
      "wireframe",
    ]
  }
};
