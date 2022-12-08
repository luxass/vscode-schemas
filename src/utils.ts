import { colors, satisfies } from "./deps.ts";

export async function isDirectoryEmpty(dir: string): Promise<boolean> {
  try {
    const items = [];
    for await (const dirEntry of Deno.readDir(dir)) {
      items.push(dirEntry);
    }

    return items.length === 0;
  } catch (error) {
    if (!(error instanceof Deno.errors.NotFound)) {
      throw error;
    }
    return true;
  }
}

export type CommandGlobalOptions = {
  release: string;
  codeSrc: string | undefined;
  out: string | undefined;
};

export async function checkVersion(bin: string, name: string, range: string) {
  const versionCommand = new Deno.Command(bin, {
    args: ["--version"]
  });
  const { code, stdout, stderr } = await versionCommand.output();
  if (code !== 0) {
    console.error(
      colors.red(`Failed to find ${name} version - received following error`)
    );
    const err = new TextDecoder().decode(stderr);
    console.error(err);
    return false;
  }
  if (!satisfies(new TextDecoder().decode(stdout), range)) {
    console.error(colors.red(`VSCode requires ${name} version ${range}`));
    return false;
  }
  return true;
}
