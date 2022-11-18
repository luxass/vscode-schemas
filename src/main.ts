import { config } from 'https://deno.land/std@0.165.0/dotenv/mod.ts';
import puppeteer from 'https://deno.land/x/puppeteer@16.2.0/mod.ts';
import { which } from 'https://deno.land/x/which@0.2.1/mod.ts';
import VERSION_FILE from '../version.json' assert { type: 'json' };
import { SPECIFIC_RELEASE_QUERY, FILES_QUERY } from './queries.ts';
import { FilesResponse, ReleaseResponse } from './types.ts';

const env = await config();
const { version } = VERSION_FILE;

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
      Authorization: `bearer ${env.GITHUB_TOKEN}`
    }
  });

  return (await res.json()) as R;
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

  const isServerInstalled = await which('code-server');

  console.log('isServerInstalled', isServerInstalled);

  if (!isServerInstalled) {
    const setupCommand = new Deno.Command('wget', {
      args: ['-q', '-O-', 'https://aka.ms/install-vscode-server/setup.sh']
    });

    const { stdout: setupStdout } = await setupCommand.output();

    if (!setupStdout) throw new TypeError('setupStdout is undefined or null');
    const text = new TextDecoder().decode(setupStdout);

    const installCommand = new Deno.Command('sh', {
      args: ['-c', text]
    });

    const { stdout: installStdout } = await installCommand.output();

    console.log(new TextDecoder().decode(installStdout));
  }

  const startCommand = new Deno.Command('sh', {
    args: [
      '-c',
      'code-server serve-local --disable-telemetry --without-connection-token --accept-server-license-terms --host 0.0.0.0 --start-server --install-extension extraction/schema-extractor-0.0.1.vsix'
    ]
  });

  startCommand.spawn();
  await delay(8000);

  const browser = await puppeteer.launch({
    product: 'firefox',
    headless: false
  });
  const page = await browser.newPage();
  await page.goto('http://localhost:8000');

  await delay(5000)
  await page.keyboard.down('Control');
  await page.keyboard.down('p');

  await page.screenshot({
    path: 'example.png'
  });
  await delay(10000)
  await browser.close();
  console.log('done');

  // Close the server after 3 seconds.
  await delay(3000);

  startCommand.kill();
}

function delay(time: number) {
  return new Promise(function (resolve) {
    setTimeout(resolve, time);
  });
}

await run();
