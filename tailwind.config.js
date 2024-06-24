module.exports = {
    content: [
      "*.html",
      "./src/**/*.rs"
    ],
    theme: {
      extend: {},
    },
    plugins: [require("daisyui")],
    daisyui: {
      themes: ["light", "dark"], // You can customize this
    },
  }