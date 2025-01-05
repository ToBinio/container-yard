// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    runtimeConfig: {
        public: {
            baseURL: "http://localhost:8080"
        }
    },
    compatibilityDate: '2024-11-01',
    devtools: {enabled: true}
})
