// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ["@unocss/nuxt", "@nuxt/devtools", "@vueuse/nuxt"],
  devtools: { enabled: true },
  plugins: [
    {
      src: "~/plugins/vercel-analytics.ts",
      mode: "client"
    }
  ]
});
