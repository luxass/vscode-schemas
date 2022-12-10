import { Command } from "../deps.ts";
import { CommandGlobalOptions, getArchitechture } from "../utils.ts";
export const devCommand = new Command<CommandGlobalOptions>()
  .description("List possible releases or schemas")
  .action(async ({
    arch
  }) => {
    console.log(arch);
    console.log(await getArchitechture());
  });
