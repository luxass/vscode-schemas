import {
  defineConfig,
  presetIcons,
  presetUno,
  transformerDirectives,
  transformerVariantGroup,
} from "unocss";

export default defineConfig({
  presets: [
    presetUno({
      dark: "media",
    }),
    presetIcons({
      scale: 1.5,
      extraProperties: {
        "color": "inherit",
        "min-width": "1.5em",
      },
    }),
  ],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
});
