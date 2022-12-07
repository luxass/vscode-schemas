import { which } from "https://deno.land/x/which@0.2.1/mod.ts";
import { error } from "./log.ts";

/**
 * Find the path to the git binary.
 * @returns The path to the git binary or undefined if not found.
 */
export const getGitBin = async () => await which("git");

/**
 * Clone a GitHub Repository
 * @param out The path to clone the repository to.
 */
export async function clone(out: string) {
  const { status } = await git([
    "clone",
    "https://github.com/microsoft/vscode.git",
    out
  ]);

  if (!status) {
    throw new Error(`Unsuccessful response for 'git clone'.`);
  }
}

export async function hasChanges(out: string) {
  const { status, output } = await git(["status", "--porcelain"], {
    path: out,
  });
  return status && output.length > 0;
}

/**
 * Checkout a release.
 * @param release The release to checkout
 * @param out Directory to checkout the release in.
 * @returns 
 */
export async function checkout(release: string, out: string) {
  const { status } = await git(["checkout", release], {
    path: out,
    throwOnFailure: false
  });

  if (!status) {
    error(`Tag ${release} was not found, are you sure it exists?`);
    return;
  }
}

export async function fetchTags(out: string) {
  const { status } = await git(["fetch", "--tags"], {
    path: out
  });

  if (!status) {
    throw new Error(`Unsuccessful response for 'git fetch --tags'.`);
  }
}

/**
 * Check if a git repository is initialized.
 * @param path The path to the git repository.
 * @returns True if the repository is initialized, false otherwise.
 */
export async function isInitialized(path: string) {
  const { status } = await git(["status"], {
    path,
    throwOnFailure: false
  });

  return status;
}

type GitResult = {
  status: boolean;
  output: string;
};

/**
 * Run a git command.
 * @param cmd The git command to run.
 * @param path The directory to run the command in.
 * @returns The status and output of the command.
 */
async function git(
  cmd: string[],
  options?: Partial<{
    path: string;
    throwOnFailure: boolean;
  }>
): Promise<GitResult> {
  const gitBin = await getGitBin();
  if (!gitBin) {
    throw new Error("Git is not installed.");
  }

  const process = Deno.run({
    cmd: [gitBin, ...cmd],
    cwd: options?.path ?? Deno.cwd(),
    stdout: "piped",
    stderr: "piped"
  });

  let output = "";

  await readLines([process.stdout, process.stderr], true, (token) => {
    output += token;
  });

  const status = await process.status();

  if (!status.success && (options?.throwOnFailure ?? true)) {
    throw new Error(`Command '${cmd.join(" ")}' has failed.`);
  }

  return {
    status: status.success,
    output: output
  };
}

async function readLines(
  readers: (Deno.Reader & Deno.Closer)[],
  closeAfterUse: boolean,
  fn: (token: string) => void
): Promise<void> {
  readers = [...readers];
  let lineBuffer = "";

  const buf = new Uint8Array(32 * 1024);

  while (readers.length > 0) {
    const [n, reader] = await Promise.race(
      readers.map((r) => readIntoBuffer(r, buf))
    );

    if (n !== null && n > 0) {
      let readStr = new TextDecoder().decode(buf.subarray(0, n));
      let lineBreak = readStr.indexOf("\n", 0);

      while (lineBreak !== -1) {
        lineBuffer += readStr.substr(0, lineBreak);
        if (lineBuffer.length > 0) {
          fn(lineBuffer);
        }
        fn("\n");
        lineBuffer = "";
        readStr = readStr.substr(lineBreak + 1);
        lineBreak = readStr.indexOf("\n", 0);
      }

      if (readStr.length > 0) {
        lineBuffer += readStr;
      }
    } else if (n === null) {
      if (lineBuffer.length > 0) {
        fn(lineBuffer);
      }

      readers = readers.filter((r) => r !== reader);
      if (closeAfterUse) {
        reader.close();
      }
    }
  }
}

async function readIntoBuffer<T extends Deno.Reader>(
  reader: T,
  buffer: Uint8Array
): Promise<[number | null, T]> {
  const n = await reader.read(buffer);
  return [n, reader];
}
