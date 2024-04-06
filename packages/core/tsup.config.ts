import {
  defineConfig,
} from 'tsup'

export default defineConfig({
  entry: [
    './src/index.ts',
    './src/patches/*.ts',
  ],
  format: ['esm'],
  clean: true,
  treeshake: true,
  dts: true,
})
