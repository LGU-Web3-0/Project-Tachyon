const gulp = require("gulp");
const webpack = require('webpack-stream');
var ts = require("gulp-typescript");
var tsProject = ts.createProject("tsconfig.json");
gulp.task("default", function () {

  return tsProject
    .src()
    .pipe(tsProject())
    .js
    .pipe(gulp.dest("dist"))
    .pipe(webpack( require("./webpack.config.js") ))
    .pipe(gulp.dest("dist"));
});
