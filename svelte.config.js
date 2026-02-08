import adapter from '@sveltejs/adapter-static';
import { sveltePreprocess } from 'svelte-preprocess';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: [sveltePreprocess(), vitePreprocess()],

	kit: {
	    appDir: 'app',
		adapter: adapter({
			fallback: '404.html'
		}),
		paths: {
			base: process.argv.includes('dev') ? '' : '/SalvarMateria'
		}
	}
};

console.log(`[DEBUG] process.env.BASE_PATH: ${process.env.BASE_PATH}.`);


export default config;
