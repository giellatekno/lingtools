import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

// If the app should live under a subpath on the domain, such as
// e.g. some.domain.com/subdir - then set this to "/subdir"
const base = process.env.SK_BASE || "";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    // Consult https://kit.svelte.dev/docs/integrations#preprocessors
    // for more information about preprocessors
    preprocess: [vitePreprocess()],

    kit: {
        alias: {
            $components: "src/components",
            $assets: "src/assets",
            $locales: "src/locales",
        },
        paths: {
            base,
        },

        // adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
        // If your environment is not supported or you settled on a specific environment, switch out the adapter.
        // See https://kit.svelte.dev/docs/adapters for more information about adapters.
        adapter: adapter(),
    },
};

export default config;
