// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  devtools: { enabled: true },

  modules: [
    "@nuxt/a11y",
    "@nuxt/eslint",
    "@nuxt/hints",
    "@nuxt/image",
    "@nuxt/ui",
    "@artmizu/nuxt-prometheus",
    "@nuxtjs/i18n",
  ],

  css: ["~/assets/css/main.css"],

  runtimeConfig: {
    public: {
      apiBaseUrl:
        process.env.NUXT_PUBLIC_API_BASE_URL || "http://localhost:8080",
    },
  },

  colorMode: {
    preference: "system",
    classSuffix: "",
  },

  i18n: {
    locales: [
      { code: "en", name: "English", file: "en.json" },
      { code: "id", name: "Indonesia", file: "id.json" },
    ],
    defaultLocale: "en",
    langDir: "locales",
    strategy: "no_prefix",
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: "i18n_redirected",
      redirectOn: "root",
    },
  },
});
