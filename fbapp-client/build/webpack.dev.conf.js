"use strict";

const FriendlyErrorsPlugin = require("friendly-errors-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const merge = require("webpack-merge");

const config = require("../config");
const utils = require("./utils");
const baseWebpackConfig = require("./webpack.base.conf");

// add hot-reload related code to entry chunks
Object.keys(baseWebpackConfig.entry).forEach(function (name) {
    baseWebpackConfig.entry[name] = ["./fbapp-client/build/dev-client"].concat(baseWebpackConfig.entry[name]);
});

module.exports = merge(baseWebpackConfig, {
    module: {
        rules: utils.styleLoaders({ sourceMap: config.dev.cssSourceMap })
    },

    // cheap-module-eval-source-map is faster for development
    devtool: "#cheap-module-eval-source-map",

    plugins: [
        new webpack.DefinePlugin({
            "process.env": config.dev.env
        }),

        // https://github.com/glenjamin/webpack-hot-middleware#installation--usage
        new webpack.HotModuleReplacementPlugin(),

        new webpack.NoEmitOnErrorsPlugin(),

        // https://github.com/ampedandwired/html-webpack-plugin
        new HtmlWebpackPlugin({
            filename: "index.html",
            template: "fbapp-client/index.html",
            inject: true
        }),

        new FriendlyErrorsPlugin()
    ]
});
