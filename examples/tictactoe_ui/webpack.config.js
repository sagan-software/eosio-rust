const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");

module.exports = {
	entry: path.resolve(__dirname, "src", "index.js"),
	output: {
		path: path.resolve(__dirname, "..", "..", "docs", "tictactoe"),
		filename: "index.js",
	},
	plugins: [
		new HtmlWebpackPlugin(),
		// Have this example work in Edge which doesn't ship `TextEncoder` or
		// `TextDecoder` at this time.
		// new webpack.ProvidePlugin({
		// 	TextDecoder: ["text-encoding", "TextDecoder"],
		// 	TextEncoder: ["text-encoding", "TextEncoder"],
		// }),
	],
	mode: "development",
};
