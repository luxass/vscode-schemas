import { defineCommand } from "citty";

export default defineCommand({
  meta: {
    name: "prebuilt",
    description: "Manage code-prebuilt",
  },
  args: {},
  subCommands: {
    "build": () => import("./build").then((r) => r.default || r),
    "prepare": () => import("./prepare").then((r) => r.default || r),
    "patch": () => import("./patch").then((r) => r.default || r),
    "list-patches": () => import("./list-patches").then((r) => r.default || r),
  },
});
