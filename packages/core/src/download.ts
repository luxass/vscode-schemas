import { createWriteStream, existsSync } from "node:fs";
import { Stream } from "node:stream";
import { promisify } from "node:util";
import { tmpdir } from "node:os";
import { join } from "node:path";
import tar from "tar";
import {
  $fetch
} from "ofetch";
import type { Release } from "./releases";

const pipeline = promisify(Stream.pipeline);

export type DownloadOptions = {
  /**
   * The directory to download the source code to.
   * @default "vscode-src"
   */
  outDir?: string
};

export async function downloadCodeSource(release: Release, {
  outDir = "vscode-src"
}: DownloadOptions) {

  if (!outDir) outDir = "vscode-src";

  if (existsSync(outDir)) {
    throw new Error("The output directory already exists, please remove it or specify a different one.");
  }

  // https://github.com/microsoft/vscode/archive/refs/tags/1.45.0.tar.gz
  // https://codeload.github.com/microsoft/vscode/tar.gz/refs/tags/1.45.0
  const tarFile = await $fetch(`https://codeload.github.com/microsoft/vscode/tar.gz/refs/tags/${release}`, {
    responseType: "blob"
  });

  const tmpFile = join(tmpdir(), `vscode-src-${release}.tar.gz`);
  await pipeline(tarFile.stream(), createWriteStream(tmpFile));

  await tar.x({
    file: tmpFile,
    cwd: outDir
  });

}
