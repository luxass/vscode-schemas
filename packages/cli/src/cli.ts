import cac from "cac";

declare const VERSION: string;

const cli = cac("vscode-schema");

cli.command("init", "Initialize a new schema project");

cli.help();
cli.version(VERSION);

cli.parse();
