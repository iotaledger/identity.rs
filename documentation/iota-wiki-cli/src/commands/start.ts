import {Command, flags} from '@oclif/command'
import {spawn} from 'child_process'
import {join, resolve} from 'path'
const  replaceInFile = require('replace-in-file')
import {readFileSync} from 'fs'
import {copySync} from 'fs-extra'
const syncDirectory = require('sync-directory')
const debounce = require('lodash.debounce')
import {getLocalConfig} from '../local-config'

export default class Start extends Command {
  static description = 'start local wiki'

  static flags = {
    help: flags.help({char: 'h'}),
  }

  async run() {
    const PWD = process.env.PWD ?? ''
    const userConfig = await getLocalConfig()
    const WIKI_GIT_FOLDER = join(PWD, userConfig.localWikiFolder, 'iota-wiki')
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

    const WIKI_CONTENT_REPO_FOLDER = join(WIKI_EXTERNAL_FOLDER, userConfig.repoName)

    copySync(join(PWD, 'static', 'img'), join(WIKI_GIT_FOLDER, 'static', 'img'))

    log(resolve(join(PWD, '..')))

    const runYarn = debounce(() => {
      spawn('yarn', [
        'start',
        '--host',
        '0.0.0.0',
      ], {
        cwd: WIKI_GIT_FOLDER,
        shell: true,
        stdio: 'inherit',
      })
    }, 100)

    syncDirectory(resolve(join(PWD, '..')), resolve(WIKI_CONTENT_REPO_FOLDER), {
      exclude: userConfig.excludeList,
      watch: true,
      afterSync: ({type, relativePath}: {type: string; relativePath: string}) => {
        this.log(`${type}: ${relativePath}`)
        if (type === 'init:hardlink') {
          runYarn()
        }
      },
    })
  }
}
