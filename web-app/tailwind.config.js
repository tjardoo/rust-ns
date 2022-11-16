const colors = require('tailwindcss/colors')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./static/**/*.html"
  ],
  theme: {
    fontFamily: {
      sans: ["Frutiger", "Istok Web", "sans-serif"],
    },
    extend: {
    },
    colors: {
      "yellow": {
        DEFAULT: "#FFC917",
      },
      "blue": {
        DEFAULT: "#003082",
      },
      "light-blue": {
        DEFAULT: "#0063D3",
      },
      "white": {
        DEFAULT: "#FFFFFF",
      },
      "red": {
        DEFAULT: "#DB0029",
      },
      "green": {
        DEFAULT: "#009A42",
      },
      "grey": {
        DEFAULT: "#070721",
        100: "#202037",
        200: "#39394D",
        300: "#515164",
        400: "#6A6A7A",
        500: "#838390",
        600: "#9C9CA6",
        700: "#B5B5BC",
        800: "#CDCDD3",
        900: "#E6E6E9",
        940: "#F0F0F2",
      },
    },
  },
  plugins: [],
}
