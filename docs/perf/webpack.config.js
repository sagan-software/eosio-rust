const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = (_env, argv) => {
  const distPath = path.resolve(__dirname, "..", "..", "target", "doc", "perf");
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8000
    },
    entry: "./index.js",
    output: {
      path: distPath,
      filename: "perf.js",
      webassemblyModuleFilename: "perf.wasm"
    },
    plugins: [
      new CopyWebpackPlugin([{ from: "./index.html", to: distPath }]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript"
      })
    ],
    watch: argv.mode !== "production"
  };
};
