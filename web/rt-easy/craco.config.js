const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  webpack: {
    plugins: {
      add: [
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "../../src/rt-easy-wasm"),
          outDir: path.resolve(__dirname, "./src/wasm/pkg"),
          watchDirectories: [path.resolve(__dirname, "../../src")],
        }),
      ],
    },
    configure: (webpackConfig) => {
      webpackConfig.resolve.extensions.push(".wasm");

      webpackConfig.module.rules.forEach((rule) => {
        (rule.oneOf || []).forEach((oneOf) => {
          if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
            oneOf.exclude.push(/\.wasm$/);
          }
        });
      });

      return webpackConfig;
    },
  },
};
