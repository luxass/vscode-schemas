
export const LATEST_RELEASE_QUERY = `query fetchRelease {
  repository(owner: "microsoft", name: "vscode") {
    latestRelease {
      tagName
    }
  }
}
`;

export const SPECIFIC_RELEASE_QUERY = `query fetchRelease($tagName: String!) {
  repository(owner: "microsoft", name: "vscode") {
    release(tagName: $tagName) {
      tagName
    }
  }
}
`;

export const FILES_QUERY = `query files($path: String!) {
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