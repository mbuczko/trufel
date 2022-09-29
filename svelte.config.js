import adapter from '@sveltejs/adapter-static';

// const dev = process.env.NODE_ENV === 'development';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter({
      pages: 'public',
      assets: 'public',
      fallback: 'index.html',
      precompress: false
    }),
    files: {
      hooks: {server: 'webapp/hooks'},
      lib: 'webapp/lib',
      routes: 'webapp/routes',
      serviceWorker: 'webapp/service-worker',
      appTemplate: 'webapp/app.html'
    }
  }
};

export default config;
