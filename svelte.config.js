import adapter from '@sveltejs/adapter-static';
import { sveltePreprocess } from 'svelte-preprocess';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const dev = process.env.NODE_ENV === 'development';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: [sveltePreprocess(), vitePreprocess()],

	kit: {
	    appDir: 'App',
		adapter: adapter({
            pages: 'docs',
            assets: 'docs',
			fallback: '404.html'
		}),
		paths: {
			base: dev ? '' : '/SalvarMateria',
		}
	}
};

export default config;
