import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

export default {
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html',  // Changed from null to 'index.html' for SPA mode
      precompress: false,
      strict: true
    }),
    files: {
      serviceWorker: 'src/sw.ts'
    }
	}
}
