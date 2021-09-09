import {Command, flags} from '@oclif/command'
import {spawn} from 'child_process'
import {join} from 'path'
const  replaceInFile = require('replace-in-file')
import {readFileSync} from 'fs'
import {copySync} from 'fs-extra'
const syncDirectory = require('sync-directory')

export default class Start extends Command {
  static description = 'start local wiki'

  static flags = {
    help: flags.help({char: 'h'}),
    // flag with a value (-n, --name=VALUE)
  }

  async run() {
    const PWD = process.env.PWD ?? ''
    const WIKI_GIT_FOLDER = join(PWD, 'local', 'iota-wiki')
    const DOCUSAURUS_CONFIG_PATH = join(WIKI_GIT_FOLDER, 'docusaurus.config.js')
    const log = this.log

    const EXTERNAL_DOCS_CONFIG = readFileSync(join(PWD, 'EXTERNAL_DOCS_CONFIG'), 'utf8')
    await replaceInFile({
      files: DOCUSAURUS_CONFIG_PATH,
      from: /\/\* AUTO GENERATED EXTERNAL DOCS CONFIG \*\//,
      to: EXTERNAL_DOCS_CONFIG,
    })

    const EXTERNAL_DOCS_DROPDOWN_CONFIG = readFileSync(join(PWD, 'EXTERNAL_DOCS_DROPDOWN_CONFIG'), 'utf8')
    await replaceInFile({
      files: DOCUSAURUS_CONFIG_PATH,
      from: /\/\* AUTO GENERATED EXTERNAL DOCS DROPDOWN CONFIG \*\//,
      to: EXTERNAL_DOCS_DROPDOWN_CONFIG,
    })

    const WIKI_EXTERNAL_FOLDER = join(WIKI_GIT_FOLDER, 'external')

    const WIKI_CONTENT_REPO_FOLDER = join(WIKI_EXTERNAL_FOLDER, 'identity.rs')

    copySync(join(PWD, 'static', 'img'), join(WIKI_GIT_FOLDER, 'static', 'img'))

    syncDirectory(join(PWD, '..'), WIKI_CONTENT_REPO_FOLDER, {
      exclude: ['local', 'wiki-cli', 'node_modules', 'target', '.git'],
      watch: true,
      afterSync({type, relativePath}) {
        log(`${type}: ${relativePath}`)
      },
    })

    setTimeout(() => {
      const yarnProcess = spawn('yarn', [
        'start',
        '--host',
        '0.0.0.0',
      ], {
        cwd: WIKI_GIT_FOLDER,
        shell: true,
      })
      yarnProcess.stdout.on('data', function (data) {
        log(data.toString())
      })
    }, 1000)
  }
}
