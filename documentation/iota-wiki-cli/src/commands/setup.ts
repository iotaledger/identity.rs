import {Command, flags} from '@oclif/command'
import {spawnSync} from 'child_process'
import {existsSync, mkdirSync, readdirSync} from 'fs'
import {getLocalConfig} from '../local-config'
import {join} from 'path'

export default class Setup extends Command {
  static description = 'setup local wiki'

  static examples = [
    '$ iota-wiki-cli setup --ref main',
  ]

  static flags = {
    help: flags.help({char: 'h'}),
    ref: flags.string({char: 'r', description: 'wiki revison to checkout'}),
  }

  async run() {
    const {flags} = this.parse(Setup)
    const ref = flags.ref ?? ''
    const PWD = process.env.PWD ?? ''
    const userConfig = await getLocalConfig()
    const WORKING_FOLDER = userConfig.localWikiFolder
    this.log(`Working in ${join(PWD, WORKING_FOLDER)}`)
    if (!existsSync(WORKING_FOLDER)) {
      mkdirSync(WORKING_FOLDER)
    }
    const GIT_ARGS = ['clone']
    if (ref) {
      GIT_ARGS.push('--branch',
        ref,)
    }
    GIT_ARGS.push('https://github.com/iota-community/iota-wiki.git')
    spawnSync('git', GIT_ARGS, {
      cwd: join(PWD, WORKING_FOLDER),
      shell: true,
      stdio: 'inherit',
    })
    const WIKI_GIT_FOLDER = join(join(PWD, WORKING_FOLDER), readdirSync(join(PWD, WORKING_FOLDER))[0])
    const WIKI_EXTERNAL_FOLDER = join(WIKI_GIT_FOLDER, 'external')
    if (!existsSync(WIKI_EXTERNAL_FOLDER)) {
      mkdirSync(WIKI_EXTERNAL_FOLDER)
    }
    const WIKI_CONTENT_REPO_FOLDER = join(WIKI_EXTERNAL_FOLDER, userConfig.repoName)
    if (!existsSync(WIKI_CONTENT_REPO_FOLDER)) {
      mkdirSync(WIKI_CONTENT_REPO_FOLDER)
    }
    const WIKI_CONTENT_DOCS_FOLDER = join(WIKI_CONTENT_REPO_FOLDER, userConfig.contentFolder)
    if (!existsSync(WIKI_CONTENT_DOCS_FOLDER)) {
      mkdirSync(WIKI_CONTENT_DOCS_FOLDER)
    }

    spawnSync('yarn', {
      cwd: WIKI_CONTENT_REPO_FOLDER,
      shell: true,
      stdio: 'inherit',
    })
  }
}
