import tailwindcss from "@tailwindcss/vite";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  future: {
    compatibilityVersion: 4,
  },
  runtimeConfig: {
    public: {
      apiURL: "http://localhost:8081",
    },
  },
  css: ["~/assets/css/main.css"],
  compatibilityDate: "2025-05-20",
  devtools: { enabled: true },
  modules: [
    "@nuxt/icon",
    "@vueuse/nuxt",
    "@nuxt/eslint",
    "@pinia/nuxt",
  ],
  vite: {
    plugins: [tailwindcss()],
  },
});