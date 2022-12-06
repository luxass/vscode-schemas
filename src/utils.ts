import { colors, Confirm, Input, satisfies, which } from "./deps.ts";

export type CommandGlobalOptions = {
  release: string;
  codeSrc: string | undefined;
  dir: string | undefined;
};

export async function downloadCodeSource(
  release: string,
  dir: string | undefined
): Promise<string> {
  console.log("No VSCode Source Code provided");
  if (!dir) {
    const wantToDownload = await Confirm.prompt("Do you want to download?");

    if (!wantToDownload) {
      console.log(colors.red("Aborting."));
      Deno.exit(1)
    }
    dir = await Input.prompt({
      message: "Where do you want to download VSCode Source Code?",
      default: "vscode"
    });
  }
  console.log("Downloading VSCode Source Code");
  const gitBin = await which("git");
  if (!gitBin) {
    console.log(colors.red("Git is not installed"));
    Deno.exit(1);
  }
  const command = new Deno.Command(gitBin, {
    args: [
      "clone",
      "--depth",
      "1",
      "--branch",
      release || "main",
      "https://github.com/microsoft/vscode.git",
      dir
    ]
  });
  const { code, stderr } = await command.output();
  // For some odd reason, git writes success to stderr. -.-
  if (code !== 0 /*|| stderr.length > 0*/) {
    console.error(
      colors.red("Failed to download VSCode - received following error")
    );
    const err = new TextDecoder().decode(stderr);
    console.error(err);
    Deno.exit(1);
  }
  return dir;
}

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
