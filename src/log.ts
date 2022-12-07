import { colors } from "./deps.ts";

export const error = (message: string) =>
  console.error(`${colors.red("ERR: ")} ${message}`);
export const warn = (message: string) =>
  console.warn(`${colors.yellow("WARN:")} ${message}`);
export const info = (message: string) =>
  console.info(`${colors.blue("INFO:")} ${message}`);
export const success = (message: string) =>
  console.info(`${colors.green("OK:  ")} ${message}`);
