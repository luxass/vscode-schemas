import { defineCommand } from 'citty'
import semver from 'semver'
import { consola } from 'consola'

export default defineCommand({
  meta: {
    name: 'download',
    description: 'Download VSCode Schemas',
  },
  args: {
    release: {
      type: 'positional',
      description: 'Release',
    },
    language: {
      type: 'string',
      description: 'Schema language to use',
      default: 'en',
    },
    out: {
      type: 'string',
      description: 'Outdir to place the schema files in',
      default: '.vss',
    },
  },
  async run(ctx) {
    let release = ctx.args.release

    if (!release) {
      consola.info('No release specified, fetching latest release.')
      release = await fetch('https://vscode.luxass.dev/releases/latest').then((res) => res.text())
    }

    if (!semver.gte(release, '1.45.0')) {
      // set release to lastest, and notify user
      consola.warn('The release you specified is not supported, using latest instead.')
      release = await fetch('https://vscode.luxass.dev/releases/latest').then((res) => res.text())
    }

    consola.info(`downloading schemas for release ${release}...`)
  },
})
