/** @type { import("snowpack").SnowpackUserConfig } */
module.exports = {
    extends: 'electron-snowpack/config/snowpack.js',
    packageOptions: {
        sourceMap: true,
    },
    buildOptions: {
        sourcemaps: true,
    },
};