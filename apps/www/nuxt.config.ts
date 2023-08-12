// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@unocss/nuxt", "@nuxt/devtools", "@vueuse/nuxt", "nuxt-icon"],
  devtools: { enabled: true },
  plugins: [
    {
      src: "~/plugins/vercel-analytics.ts",
      mode: "client",
    },
  ],
  css: [
    "@unocss/reset/tailwind.css",
  ],
  // routeRules: {
  //   "/api/search": {
  //     prerender: true,
  //   },
  // },
  app: {
    head: {
      htmlAttrs: {
        lang: "en",
      },
    },
  },
});
