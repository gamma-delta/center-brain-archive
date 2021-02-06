const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: './src/main.ts',
    output: {
        filename: 'center_brain_archive.js',
        path: path.resolve(__dirname, 'generated'),
    },
    mode: 'development',
    devtool: 'source-map',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: "src/*.html", to: "[name].[ext]" },
                { from: "src/*.css", to: "[name].[ext]" },
                { from: "src/dsp.json", to: "dsp.json" },
            ]
        })
    ],

};