import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  future: {
    compatibilityVersion: 4,
  },
  runtimeConfig: {
    public: {
      baseURL: "http://localhost:8080",
    },
  },
  compatibilityDate: "2025-05-20",
  devtools: { enabled: true },
  modules: ["@nuxt/icon", "@vueuse/nuxt", "@nuxt/eslint"],
  vite: {
    plugins: [tailwindcss()],
  },
});
