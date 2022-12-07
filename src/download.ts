import { Confirm, Input } from "./deps.ts";
import {
  checkout,
  clone,
  fetchTags,
  hasChanges,
  isInitialized
} from "./git.ts";
import { info, warn } from "./log.ts";
import { isDirectoryEmpty } from "./utils.ts";

type DownloadOptions = {
  out?: string;
  codeSrc?: string;
};

export async function downloadCodeSource(
  release: string,
  options: DownloadOptions
): Promise<string> {
  const { out, codeSrc } = options;

  let path: string | undefined = codeSrc || out;
  if (!codeSrc && !out) {
    const wantToDownload = await Confirm.prompt("Do you want to download?");

    if (!wantToDownload) {
      info("Aborting.");
      Deno.exit(1);
    }
    path = await Input.prompt({
      message: "Where do you want to download VSCode Source Code?",
      default: "vscode"
    });
  }

  if (!path) throw new TypeError("No path provided. Please report this");
  
  const isOutEmpty = await isDirectoryEmpty(path);
  if (isOutEmpty) {
    info("Downloading VSCode Source Code");
    await clone(path);
    await checkout(release, path);
  } else {
    const isGitRepo = await isInitialized(path);

    if (isGitRepo) {
      await fetchTags(path);
      const changes = await hasChanges(path);
      if (changes) {
        warn("Changes found. Aborting.");
        info("Please commit or stash your changes first.");
        Deno.exit(1);
      }
      await checkout(release, path);
    }
  }

  return "path";
}
