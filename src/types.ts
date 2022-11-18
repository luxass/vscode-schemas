
export interface ReleaseResponse {
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

interface Object {
  entries: Entry[];
}

interface Entry {
  name: string;
  object: {
    text: string;
  };
}

export interface FilesResponse {
  data: {
    repository: {
      object: Object;
    };
  };
}