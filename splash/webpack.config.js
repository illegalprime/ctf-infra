var path = require("path");

module.exports = {
    entry: "./js/splash.js",
    output: {
        path: __dirname,
        filename: "bundle.js",
    },
    module: {
        loaders: [
            {
                test: /\.js$/,
                exclude: /node_modules/,
                loader: "babel-loader",
            },
            {
                test: /\.json$/,
                exclude: /node_modules/,
                loader: "json-loader",
            },
        ],
    },
    resolve: {
        modulesDirectories: [
            "node_modules",
            "bower_components",
        ],
    },
};

