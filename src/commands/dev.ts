import { Command } from "../deps.ts";
import { CommandGlobalOptions, detectArch } from "../utils.ts";

export const devCommand = new Command<CommandGlobalOptions>()
  .description("List possible releases or schemas")
  .action(async ({
    arch
  }) => {
    console.log("Hello world");
    console.log(arch);
    
    console.log(await detectArch());
    
  });
