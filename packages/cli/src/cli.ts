import { existsSync } from 'node:fs'
import { writeFile } from 'node:fs/promises'
import process from 'node:process'
import cac from 'cac'
import semver from 'semver'
import type { Release } from '@vscode-schemas/core'
import {
  downloadCodeSource,
  scan,
} from '@vscode-schemas/core'
import { bold, green, inverse, red, yellow } from 'colorette'
import { defineCommand, runMain } from 'citty'

import pkg from '../package.json' with { type: 'json' }
import { commands } from './commands'
// cli.command('download-src [release] [out]', 'Download VSCode Source Code')
//   .option('--out [out]', 'Outdir to place the source code', {
//     default: '.vscode-src',
//   })
//   .option('-f, --force', 'Force download source code (will delete files in out)', {
//     default: false,
//   })
//   .action(async (release: string, out: string, options: GlobalCLIOptions & {
//     force: boolean
//   }) => {
//     if (!release) {
//       release = await fetch('https://latest-vscode-release.luxass.dev').then((res) => res.text())
//     }

//     if (!semver.gte(release, '1.45.0')) {
//       // set release to lastest, and notify user
//       console.warn('The release you specified is not supported, using latest instead.')
//       release = await fetch('https://latest-vscode-release.luxass.dev').then((res) => res.text())
//     }
//     try {
//       await downloadCodeSource(release as Release, {
//         out: out || options.out,
//         force: options.force || false,
//       })
//       console.warn(`Downloaded source code to ${green(out || options.out || '.vscode-src')}`)
//     } catch (err) {
//       if (typeof err === 'string') {
//         console.error(err)
//       }

//       if (err instanceof Error && err.message === `outDir "${out || options.out}" is not empty`) {
//         console.error(
//           `The outDir "${out || options.out}" is not empty, use --force to force download source code.`,
//         )
//         return
//       }

//       throw err
//     }
//   })

const main = defineCommand({
  meta: {
    name: pkg.name,
    version: pkg.version,
    description: pkg.description,
  },
  subCommands: commands,
})

runMain(main)
