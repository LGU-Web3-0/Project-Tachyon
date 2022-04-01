const gulp = require("gulp");
const webpack = require('webpack-stream');
var ts = require("gulp-typescript");
var tsProject = ts.createProject("tsconfig.json");
gulp.task("default", function () {

  return tsProject
    .src()
    .pipe(tsProject())
    .js
    .pipe(gulp.dest(".tmp"))
    .pipe(webpack( require("./webpack.config.js") ))
    .pipe(gulp.dest("dist"));
});

gulp.task('css', () => {
  const postcss    = require('gulp-postcss')
  const sourcemaps = require('gulp-sourcemaps')

  return gulp.src('src/**/*.css')
      .pipe( sourcemaps.init() )
      .pipe( postcss() )
      .pipe( sourcemaps.write('.') )
      .pipe( gulp.dest('dist') )
})
