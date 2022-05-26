module.exports = {
  presets: [
    "@babel/preset-react",
    [
      "@babel/preset-env",
      {
        targets: {
          node: "current",
        },
      },
    ],
  ],
  plugins: [
    [
      "@babel/plugin-proposal-private-property-in-object",
      {
        loose: true,
      },
    ],
  ],
}
