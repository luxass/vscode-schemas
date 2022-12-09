#!/usr/bin/env -S deno run --allow-read --allow-write --allow-env --allow-net --allow-run --unstable
import { octokit, SemVer } from "../src/deps.ts";
import { downloadCodeSource } from "../src/internal/download.ts";
import { info } from "../src/log.ts";

async function run() {
  const args = Deno.args;

  const codeSrc = args[0] || "vscode" 
  
  console.log("Checking for downloads...");

  await downloadCodeSource("main", {
    out: codeSrc,
    skipCheckout: true
  });
  const releases = (
    await octokit.paginate("GET /repos/{owner}/{repo}/releases", {
      owner: "microsoft",
      repo: "vscode"
    })
  )
    .filter((release) => {
      const version = new SemVer(release.tag_name);
      return version.major >= 1 && version.minor >= 45;
    })
    .map((release) => release.tag_name).reverse();

  info(`Found ${releases.length} releases`);
  for (const release of releases) {
    info(`Running for ${release}`);

  const p = Deno.run({
    cmd: [
      "./src/main.ts",
      "scan",
      "--code-src",
      codeSrc,
      "--release",
      release,
      "--default-out"
    ],
    stdout: "inherit",
    stderr: "inherit"
  });

    const status = await p.status();
    if (!status.success) {
      Deno.exit(status.code);
    }
  }
}

await run();
