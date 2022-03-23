module.exports = {
    "ui": {
        "port": 8081
    },
    "files": [
        "./**/*.wasm",
        "./**/*.hbs",
        "./**/*.css",
    ],
    "proxy": {
        target: "http://localhost:3000"
    },
    "port": 8080,
    "injectChanges": false,
    "reloadDelay": 1000
};