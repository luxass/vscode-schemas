import { Octokit as _Octokit } from "https://esm.sh/@octokit/core@4.1.0";
import { restEndpointMethods } from "https://esm.sh/@octokit/plugin-rest-endpoint-methods@6.7.0";
import { paginateRest } from "https://esm.sh/@octokit/plugin-paginate-rest@5.0.1";


const Octokit = _Octokit.plugin(restEndpointMethods, paginateRest);
const octokit = new Octokit({
  auth: Deno.env.get("GITHUB_TOKEN")
});

export { Octokit, restEndpointMethods, paginateRest, octokit };
