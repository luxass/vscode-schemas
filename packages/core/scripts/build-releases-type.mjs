// @ts-check
import "dotenv/config";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import { $fetch } from "ofetch"
async function run() {
  const releases = await $fetch("https://vscode-releases.luxass.dev", {
    responseType: "blob"
  })
    .then((blob) => blob.text())
    .then((text) => JSON.parse(text))
    .then((json) => json.map((/** @type {{ name: string; url: string }} */ release) => `"${release.name}"`));

  console.log(releases);

    await writeFile(join(new URL("..", import.meta.url).pathname, "./src/releases.ts"), `/**
 * This file is prebuilt from packages/core/scripts/build-releeases-type.mjs
 * Do not edit this directly, but instead edit that file and rerun it to generate this file.
 */

export type Release = ${releases.reverse().join(" | ")};
`);
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
