const { series, src, dest, watch } = require('gulp');
const inject = require('gulp-inject-string');

const browserSync = require('browser-sync').create();

function startBrowserSync(done) {
  browserSync.init(
    {
      ui: false,
      localOnly: true,
      port: 35829,
      ghostMode: false,
      open: false,
      notify: false,
      logSnippet: false,
    },
    function(error, bs) {
      done(error);
    },
  );
}

function injectBrowserSync() {
  return src('src/renderer/index.html')
    .pipe(inject.before('</body>', browserSync.getOption('snippet')))
    .pipe(inject.after('script-src', ' ' + browserSync.getOption('urls').get('local')))
    .pipe(dest('build/src/renderer'));
}

function reloadBrowser(done) {
  browserSync.reload();
  done();
}

startBrowserSync.displayName = 'start-hotreload';
injectBrowserSync.displayName = 'inject-hotreload';
reloadBrowser.displayName = 'reload-hotreload';

exports.start = series(startBrowserSync, injectBrowserSync);
exports.inject = injectBrowserSync;
exports.reload = reloadBrowser;