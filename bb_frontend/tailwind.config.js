module.exports = {
  purge: [
    "./components/*.rs",
    "./pages/*.rs",
  ],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {
      cursor: ['hover'],
    },
  },
  plugins: [],
}
