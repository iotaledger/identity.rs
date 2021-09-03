import {Command, flags} from '@oclif/command'
import {spawnSync} from 'child_process'
import {join} from 'path'
const  replaceInFile = require('replace-in-file')
import {readFileSync} from 'fs'

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

    spawnSync('npm', [
      'start',
      '--',
      '--host',
      '0.0.0.0',
    ], {
      cwd: WIKI_GIT_FOLDER,
      shell: true,
      stdio: 'inherit',
    })
  }
}
