import { colors, Command, EnumType, octokit } from "../deps.ts";
import { CommandGlobalOptions } from "../utils.ts";
const show = new EnumType(["schemas", "releases"]);

export const listCommand = new Command<CommandGlobalOptions>()
  .description("List possible releases or schemas")
  .type("show", show)
  .option("-s, --show [show:show]", "Show schemas or releases")
  .option("-a, --all", "Show all releases")
  .action(async ({ show, release, all }) => {
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

      const groups = files.reduce((acc, file) => {
        if (!file.path || file.path.includes(".vscode-schemas.json"))
          return acc;

        const [group] = file.path.split("/");
        if (!acc[group]) {
          acc[group] = [];
        }
        if (file.path !== group) acc[group].push(file);
        return acc;
      }, {} as Record<string, typeof files>);

      if (all) {
        for (const [group, files] of Object.entries(groups)) {
          console.log(colors.green(group));
          for (const file of files) {
            console.log(`  ${file.path?.split("/")[1]}`);
          }
        }
      } else {
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
    }
  });
