import { Octokit as _Octokit } from "https://esm.sh/@octokit/core@4.1.0";
import { restEndpointMethods } from "https://esm.sh/@octokit/plugin-rest-endpoint-methods@6.7.0";
import { paginateRest } from "https://esm.sh/@octokit/plugin-paginate-rest@5.0.1";
import {
  EnumType,
  Command,
  colors,
  Confirm,
  Input
} from "https://deno.land/x/cliffy@v0.25.5/mod.ts";
import { SemVer, satisfies } from "https://deno.land/std@0.167.0/semver/mod.ts";
import { join, extname, dirname } from "https://deno.land/std@0.167.0/path/mod.ts";
import { walk } from "https://deno.land/std@0.167.0/fs/mod.ts";
import { which } from "https://deno.land/x/which@0.2.1/mod.ts";
import { URI } from "https://esm.sh/vscode-uri@3.0.6";

const Octokit = _Octokit.plugin(restEndpointMethods, paginateRest);
const octokit = new Octokit({
  auth: Deno.env.get("GITHUB_TOKEN")
});

export {
  Octokit,
  restEndpointMethods,
  paginateRest,
  octokit,
  EnumType,
  Command,
  Confirm,
  Input,
  colors,
  SemVer,
  satisfies,
  join,
  extname,
  walk,
  which,
  URI,
  dirname
};
