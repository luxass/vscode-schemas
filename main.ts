import VERSION_FILE from './version.json';
const { version } = VERSION_FILE;

const LATEST_RELEASE_QUERY = `query fetchRelease {
  repository(owner: "microsoft", name: "vscode") {
    latestRelease {
      tagName
    }
  }
}
`;

const SPECIFIC_RELEASE_QUERY = `query fetchRelease($tagName: String!) {
  repository(owner: "microsoft", name: "vscode") {
    release(tagName: $tagName) {
      tagName
    }
  }
}
`;

const FILES_QUERY = `query files($path: String!) {
  repository(owner: "microsoft", name: "vscode") {
    object(expression: $path) {
      ... on Tree {
        entries {
          name
          object {
            ... on Blob {
              text
            }
          }
        }
      }
    }
  }
}
`;

interface ReleaseResponse {
  data: {
    repository: {
      release?: {
        tagName: string;
      };
      latestRelease?: {
        tagName: string;
      };
    };
  };
}

export interface Object {
  entries: Entry[];
}

export interface Entry {
  name: string;
  object: {
    text: string;
  };
}

interface FilesResponse {
  data: {
    repository: {
      object: Object;
    };
  };
}

async function graphql<R>(
  query: string,
  variables?: Record<string, string>
): Promise<R> {
  const res = await fetch('https://api.github.com/graphql', {
    method: 'POST',
    body: JSON.stringify({
      query: query,
      variables
    }),
    headers: {
      'User-Agent': 'vscode-schema',
      'Content-Type': 'application/json',
      Authorization: `bearer ${process.env.GITHUB_TOKEN}`
    }
  });

  return await res.json();
}

async function run() {
  const {
    data: { repository: release }
  } = await graphql<ReleaseResponse>(SPECIFIC_RELEASE_QUERY, {
    tagName: version
  });

  if (!release.latestRelease && !release.release) {
    throw new Error('No release found');
  }

  const tagName = release.latestRelease?.tagName || release.release?.tagName;
  if (tagName === 'version' || !tagName) {
    throw new Error('No tag name found');
  }

  const files = await graphql<FilesResponse>(FILES_QUERY, {
    path: `refs/tags/${tagName}:extensions/configuration-editing`
  });

  const pkgJsonFile = files.data.repository.object.entries.find(
    (entry) => entry.name === 'package.json'
  );
  if (!pkgJsonFile) {
    throw new Error('No package.json found');
  }

  const { contributes } = JSON.parse(pkgJsonFile.object.text);

  const { jsonValidation, languages } = contributes;

  const schemas: string[] = jsonValidation
    .map((schema) => schema.url)
    // This can return undefined. FIX
    .concat(...languages.map((language) => language.filenamePatterns));

  

  console.log(schemas);
}

await run();
