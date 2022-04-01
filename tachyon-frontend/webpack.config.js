const path = require('path');

module.exports = {
    mode: "production",
    devtool: 'source-map',
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "tachyon.js",
        library: {
            type: "umd2",
            name: "Tachyon"
        }
    },
}
