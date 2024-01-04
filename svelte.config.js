import adapter from '@sveltejs/adapter-auto';
//import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			out: 'public',
			pages: 'public',
			assets: 'public',
			fallback: 'index.html',
			precompress: false
		}),
		files: {
			hooks: { server: 'webapp/hooks' },
			lib: 'webapp/lib',
			assets: 'webapp/static',
			routes: 'webapp/routes',
			serviceWorker: 'webapp/service-worker',
			appTemplate: 'webapp/app.html'
		}
	}
};

export default config;
