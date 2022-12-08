#!/usr/bin/env -S deno run --allow-read --allow-write --allow-env --allow-net --allow-run --unstable
import { octokit, SemVer } from "../src/deps.ts";
import { downloadCodeSource } from "../src/internal/download.ts";
import { info } from "../src/log.ts";

async function run() {
  console.log("Checking for downloads...");

  // await downloadCodeSource("main", {
  //   out: "/home/luxas/dev/vscode",
  //   skipCheckout: true
  // });
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
    .map((release) => release.tag_name);

  info(`Found ${releases.length} releases`);
  // for (const release of releases) {
  //   info(`Running for ${release}`);

  //   const p = Deno.run({
  //     cmd: [
  //       "./src/main.ts",
  //       "scan",
  //       "--code-src",
  //       "/home/luxas/dev/vscode",
  //       "--release",
  //       release,
  //       "--default-out"
  //     ],
  //     stdout: "inherit",
  //     stderr: "inherit"
  //   });

  //   const status = await p.status();
  //   if (!status.success) {
  //     Deno.exit(status.code);
  //   }
  // }

  await Promise.all(
    releases.map(async (release) => {
      return await new Promise((r, e) => {
        info(`Running for ${release}`);

        const p = Deno.run({
          cmd: [
            "./src/main.ts",
            "scan",
            "--code-src",
            "/home/luxas/dev/vscode",
            "--release",
            release,
            "--default-out"
          ],
          stdout: "inherit",
          stderr: "inherit"
        });

        p.status().then((status) => {
          if (!status.success) {
            e(status.code);
          }
          r(null);
        })
      });
    })
  );
}

await run();
