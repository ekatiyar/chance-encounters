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
      themes: ["dark"], // You can customize this
    },
  }