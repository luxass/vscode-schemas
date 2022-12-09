#!/usr/bin/env -S deno run --allow-read --allow-write

let README = `
# Visual Studio Code Schemas

> This is a collection of schemas for Visual Studio Code\.

## Versions
`;

async function run() {
  const cwd = Deno.cwd();
  const schemasDir = cwd.endsWith("/schemas") ? cwd : `${cwd}/schemas`;

  let schemas: Array<string> = [];
  for await (const release of Deno.readDir(schemasDir)) {
    if (release.isDirectory) {
      schemas.push(release.name);
    }
  }

  schemas = schemas.sort().reverse();
  schemas.forEach((schema) => {
    README += `- [${schema}](./schemas/${schema})\n`;
  });

  await Deno.writeTextFile(`${schemasDir}/README.md`, README.trim());
  await Deno.writeTextFile(
    `${schemasDir}/.vscode-schemas.json`,
    JSON.stringify(
      schemas.map((release) => ({
        name: release,
        url: `https://raw.githubusercontent.com/luxass/vscode-schemas/main/schemas/${release}/.vscode-schemas.json`
      })),
      null,
      2
    )
  );
}

await run();
