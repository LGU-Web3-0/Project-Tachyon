const gulp = require("gulp");
const webpack = require('webpack-stream');
const ts = require("gulp-typescript");
const tsProject = ts.createProject("tsconfig.json");
const postcss = require('gulp-postcss')
const sourcemaps = require('gulp-sourcemaps')

gulp.task("default", function () {
    const js = tsProject
        .src()
        .pipe(tsProject());
    return js
        .pipe(gulp.dest(".tmp"))
        .pipe(webpack(require("./webpack.config.js")))
        .pipe(gulp.dest("dist"));
});

gulp.task('css', () => {
    return gulp.src('src/**/*.css')
        .pipe(sourcemaps.init())
        .pipe(postcss())
        .pipe(sourcemaps.write('.'))
        .pipe(gulp.dest('dist'))
})
