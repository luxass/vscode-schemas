import { colors, join, satisfies } from "./deps.ts";

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
  release: string
  codeSrc: string | undefined
  out: string | undefined
  arch: string
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

export async function getArchitechture() {
  if (Deno.build.arch === "x86_64") {
    return "x64";
  } else if (Deno.build.arch == "aarch64") {
    return "arm64";
  }

  if (Deno.build.os === "darwin") {
    return "x64";
  }

  if (Deno.build.os === "windows") {
    const systemRoot = Deno.env.get("SystemRoot");
    const sysRoot =
      systemRoot && (await Deno.stat(systemRoot)) ? systemRoot : "C:\\Windows";

    let isWOW64 = false;
    try {
      isWOW64 = sysRoot ?
          !!(await Deno.stat(join(sysRoot, "sysnative"))) :
        false;
    } catch (e) {
      console.log(e);
    }

    return isWOW64 ? "x64" : "x86";
  }

  if (Deno.build.os === "linux") {
    const process = Deno.run({ cmd: ["getconf", "LONG_BIT"], stdout: "piped" });
    const output = new TextDecoder("utf-8").decode(await process.output());
    return output === "64\n" ? "x64" : "x86";
  }

  return "x86";
}
