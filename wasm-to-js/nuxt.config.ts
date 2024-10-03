import wasm from "vite-plugin-wasm";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2024-04-03",
    devtools: { enabled: true },
    vite: {
        plugins: [wasm()],
    },
});
