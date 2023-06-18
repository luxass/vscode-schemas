// @ts-check
import "dotenv/config";
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import {
  Octokit
} from "@octokit/core";

import {
  paginateRest
} from "@octokit/plugin-paginate-rest";
import semver from "semver";

const $Octokit = Octokit.plugin(paginateRest);
const octokit = new $Octokit({
  auth: process.env.GITHUB_TOKEN
});

async function run() {
  if (!process.env.GITHUB_TOKEN) {
    throw new Error("GITHUB_TOKEN environment variable is not set");
  }
  const releases = await octokit.paginate("GET /repos/{owner}/{repo}/releases", {
    owner: "microsoft",
    repo: "vscode",
    per_page: 100
  }).then((releases) => releases.filter((release) => semver.gte(release.tag_name, "1.45.0")));

  await writeFile(join(new URL("..", import.meta.url).pathname, "./src/releases.ts"), `
/**
* This file is prebuilt from packages/core/scripts/get-releases.mjs
* Do not edit this directly, but instead edit that file and rerun it to generate this file.
*/

export type Release = ${releases.reverse().map((release) => `"${release.tag_name}"`).join(" | ")};
`);
}

run().catch((err) => {
  console.error(err);
  process.exit(1);
});
