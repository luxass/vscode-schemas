import { readdir } from "node:fs/promises";
import { join } from "node:path";

export type ScanOptions = {
  out?: string
  scan?: "schemas" | "extension-schemas" | "both"
};

export async function scan(
  codeSrc: string,
  options?: ScanOptions
) {

  for await (const entry of walk(codeSrc)) {
    if (entry.isFile) {
      console.log(entry.path);

      // use ripgrep binary, will be published at @luxass/klow in the near future.

    }
  }
}

type Entry = {
  name: string
  isFile: boolean
  isDirectory: boolean
  path: string
};

export async function* walk(dir: string): AsyncIterableIterator<Entry> {
  try {
    const dirs = await readdir(dir, { withFileTypes: true });
    for (const entry of dirs) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) {
        yield * walk(path);
      } else {
        yield {
          name: entry.name,
          isFile: entry.isFile(),
          isDirectory: entry.isDirectory(),
          path
        };
      }
    }
  } catch (err) {
    console.error(err);
  }
}
