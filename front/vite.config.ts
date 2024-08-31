import { defineConfig } from 'vite'
import { sveltekit } from '@sveltejs/kit/vite';
import { VitePWA } from 'vite-plugin-pwa'
import { SvelteKitPWA } from '@vite-pwa/sveltekit'

// https://vitejs.dev/config/
export default defineConfig({
  envDir: './env',
  server: {
    host: true,
  },
  plugins: [
    sveltekit(),
    SvelteKitPWA({
      strategies: 'injectManifest',
      injectRegister: "auto",
      registerType: 'autoUpdate',
      devOptions: {
        enabled: true,
        type: 'module',
      },
      manifest: {
        name: 'Sim Bruna',
        short_name: 'SBruna',
        description: 'Its the Sim Bruna',
        theme_color: '#000000',
        icons: [
          {
            src: 'pwa-192x192.png',
            sizes: '192x192',
            type: 'image/png'
          },
          {
            src: 'pwa-512x512.png',
            sizes: '512x512',
            type: 'image/png'
          }
        ]
      }
    }),
  ],
  define: {
    'process.env.NODE_ENV': process.env.NODE_ENV === 'production' 
      ? '"production"'
      : '"development"'
  }
})


