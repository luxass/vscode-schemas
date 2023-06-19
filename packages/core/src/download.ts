import { existsSync } from "node:fs";
import type { Release } from "./releases";
import { clone } from "./git";

export type DownloadOptions = {
  /**
   * The directory to download the source code to.
   * @default "vscode-src"
   */
  outDir?: string
};

export async function download(release: Release, {
  outDir = "vscode-src"
}: DownloadOptions) {

  if (!outDir) outDir = "vscode-src";

  if (existsSync(outDir)) {
    throw new Error("The output directory already exists, please remove it or specify a different one.");
  }

  await clone(outDir);
}
