import {
  EnumType,
  Command,
  colors
} from "https://deno.land/x/cliffy@v0.25.5/mod.ts";
const show = new EnumType(["schemas", "releases"]);
import { octokit } from "../deps.ts";

export const listCommand = new Command<{
  release: true | string | undefined;
}>()
  .description("List possible releases or schemas")
  .type("show", show)
  .option("-s, --show [show:show]", "Show schemas or releases")
  .action(async ({ show, release }) => {
    if (show === "releases") {
      const releases = await octokit.paginate(
        "GET /repos/{owner}/{repo}/releases",
        {
          owner: "microsoft",
          repo: "vscode"
        }
      );

      for (const release of releases.reverse()) {
        console.log(colors.green(release.tag_name));
      }
    } else {
      const { data } = (await octokit.request(
        "GET /repos/{owner}/{repo}/contents/{path}",
        {
          owner: "luxass",
          repo: "vscode-schemas",
          path: ".",
          ref: "luxass/rewrite-in-deno"
        }
        // For some reason, Octokit didn't give me the correct type.
      )) as {
        data: Array<{ name: string; type: "dir" | "file"; sha: string }>;
      };

      const schemasContent = data.find(
        (d) => d.name === "schemas" && d.type === "dir"
      );

      const schemasSha = schemasContent?.sha;

      if (!schemasSha) {
        console.log("No schemas found");
        return;
      }

      const { data: schemas } = await octokit.request(
        "GET /repos/{owner}/{repo}/git/trees/{tree_sha}",
        {
          owner: "luxass",
          repo: "vscode-schemas",
          tree_sha: schemasSha,
          recursive: "1"
        }
      );

      const files = schemas.tree.filter((file) => file?.path !== "README.md");

      // @ts-ignore - just forget it.
      if (typeof release === "boolean") Deno.exit(1);

      const groups = files.reduce((acc, file) => {
        if (!file.path) return acc;

        const [group] = file.path.split("/");
        if (!acc[group]) {
          acc[group] = [];
        }
        if (file.path !== group) acc[group].push(file);
        return acc;
      }, {} as Record<string, typeof files>);

      if (release) {
        if (!groups[release]) {
          console.log("No schemas found");
          return;
        } else {
          console.log(colors.green(release));
          for (const file of groups[release]) {
            console.log(`  ${file.path?.split("/")[1]}`);
          }
          return;
        }
      }

      for (const [group, files] of Object.entries(groups)) {
        console.log(colors.green(group));
        for (const file of files) {
          console.log(`  ${file.path?.split("/")[1]}`);
        }
      }
    }
  });
