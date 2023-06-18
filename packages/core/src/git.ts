import { cwd } from "node:process";
import {
  execa
} from "execa";

import which from "which";

/**
 * Find the path to the git binary.
 * @returns The path to the git binary or undefined if not found.
 */
export const getGitBin = async () => await which("git");


export async function clone(out: string) {
  const { status } = await git([
    "clone",
    "https://github.com/microsoft/vscode.git",
    out
  ]);

  if (!status) {
    throw new Error("Unsuccessful response for 'git clone'.");
  }
}

type GitResult = {
  status: boolean
  output: string
};

/**
 * Run a git command.
 * @param cmd The git command to run.
 * @param options Options for the command.
 * @returns The status and output of the command.
 */
async function git(
  cmd: string[],
  options?: Partial<{
    path: string
    throwOnFailure: boolean
  }>
): Promise<GitResult> {
  const gitBin = await getGitBin();
  if (!gitBin) {
    throw new Error("Git is not installed.");
  }

  const process = await execa(gitBin, cmd, {
    cwd: options?.path ?? cwd(),
    stderr: "pipe",
    stdout: "pipe"
  });

  let output = "";
  console.log(process.stdout);

  await readLines([process.stdout, process.stderr], true, (token) => {
    output += token;
  });

  const status = await process.status();

  if (process.failed) {
    error(`Command '${cmd.join(" ")}' has failed.`);
    throw new Error(`Output: ${output}`);
  }

  return {
    status: status.success,
    output
  };
}
