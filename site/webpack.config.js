const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: './src/main.ts',
    output: {
        filename: 'center_brain_archive.js',
        path: path.resolve(__dirname, 'generated'),
    },
    mode: 'production',
    devtool: 'source-map',
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"],
            },
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js', '.css'],
    },
    plugins: [
        new CopyPlugin({
            patterns: [
                { from: "src/*.html", to: "[name].[ext]" },
                { from: "src/*.css", to: "[name].[ext]" },
                { from: "src/img/*.png", to: "img/[name].[ext]" },
                { from: "src/dsp.json", to: "dsp.json" },
            ]
        })
    ],

};