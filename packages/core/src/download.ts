import { createWriteStream, existsSync } from "node:fs";
import { Stream } from "node:stream";
import { promisify } from "node:util";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { mkdir, readdir, rm, unlink } from "node:fs/promises";
import tar from "tar";
import {
  $fetch
} from "ofetch";
import type { Release } from "./releases";

const pipeline = promisify(Stream.pipeline);

export type DownloadOptions = {
  /**
   * The directory to download the source code to.
   * @default ".vscode-src"
   */
  out?: string

  /**
   * Will force download the source code even if the outDir is not empty.
   *
   * **WARNING**: This will delete all files in the outDir.
   *
   * @default false
   */
  force?: boolean
};

export async function downloadCodeSource(release: Release, {
  out = ".vscode-src",
  force = false
}: DownloadOptions) {

  if (!out) out = ".vscode-src";

  if (!existsSync(out)) {
    await mkdir(out, {
      recursive: true
    });
  }

  if ((await readdir(out)).length > 0) {
    if (!force) {
      throw new Error(`outDir "${out}" is not empty`);
    }

    // delete all files in out
    await rm(out, {
      recursive: true
    });

    await mkdir(out, {
      recursive: true
    });
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
    cwd: out,
    strip: 1
  });

  await unlink(tmpFile);
}
