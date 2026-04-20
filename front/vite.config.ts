import { paraglideVitePlugin } from "@inlang/paraglide-js";
import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { enhancedImages } from "@sveltejs/enhanced-img";

export default defineConfig({
    server: {
        // needed for the local nginx to see the dev server
        host: "0.0.0.0",
    },

    plugins: [
        tailwindcss(),
        enhancedImages(),
        sveltekit(),
        paraglideVitePlugin({
            project: "./project.inlang",
            outdir: "./src/lib/paraglide",
            strategy: ["localStorage", "baseLocale"],
        }),
    ],
});
