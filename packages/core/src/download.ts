import { createWriteStream, existsSync } from "node:fs";
import { Stream } from "node:stream";
import { promisify } from "node:util";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { mkdir, readdir, rm, unlink } from "node:fs/promises";
import tar from "tar";
import {
  $fetch,
} from "ofetch";
import type { Release } from "./releases";

const pipeline = promisify(Stream.pipeline);

export interface DownloadOptions {
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
}

/**
 * Download the source code of a specific release.
 * @param {Release} release The release to download the source code from.
 * @param {DownloadOptions} options The options for downloading the source code.
 *
 * NOTE:
 * This function throws if the outDir is not empty, and force is not set to true.
 */
export async function downloadCodeSource(release: Release, options?: DownloadOptions): Promise<void> {
  if (!options) {
    options = {
      out: ".vscode-src",
      force: false,
    };
  }

  if (!options?.out) options.out = ".vscode-src";

  if (!existsSync(options.out)) {
    await mkdir(options.out, {
      recursive: true,
    });
  }

  if ((await readdir(options.out)).length > 0) {
    if (!options.force) {
      throw new Error(`outDir "${options.out}" is not empty`);
    }

    // delete all files in out
    await rm(options.out, {
      recursive: true,
    });

    await mkdir(options.out, {
      recursive: true,
    });
  }

  // https://github.com/microsoft/vscode/archive/refs/tags/1.45.0.tar.gz
  // https://codeload.github.com/microsoft/vscode/tar.gz/refs/tags/1.45.0
  const tarFile = await $fetch(`https://codeload.github.com/microsoft/vscode/tar.gz/refs/tags/${release}`, {
    responseType: "blob",
  });

  const tmpFile = join(tmpdir(), `vscode-src-${release}.tar.gz`);
  await pipeline(tarFile.stream(), createWriteStream(tmpFile));

  await tar.x({
    file: tmpFile,
    cwd: options.out,
    strip: 1,
  });

  await unlink(tmpFile);
}
