import { stat } from "node:fs/promises";
import type { Release } from "./releases";

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

  const outDirStat = await stat(outDir);

  if (outDirStat.isDirectory()) {
    throw new Error("The output directory already exists, please remove it or specify a different one.");
  }


}
