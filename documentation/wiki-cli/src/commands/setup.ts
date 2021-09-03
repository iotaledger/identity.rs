import {Command, flags} from '@oclif/command'
import {spawnSync} from 'child_process'
import {existsSync, mkdirSync, readdirSync, symlinkSync} from 'fs'
import {join} from 'path'

export default class Setup extends Command {
  static description = 'setup local wiki'

  static examples = [
    '$ wiki-cli setup --ref main',
  ]

  static flags = {
    help: flags.help({char: 'h'}),
    // flag with a value (-r, --ref=VALUE)
    ref: flags.string({char: 'r', description: 'wiki revison to checkout'}),
  }

  async run() {
    const {flags} = this.parse(Setup)
    const ref = flags.ref ?? ''
    const PWD = process.env.PWD ?? ''
    const WORKING_FOLDER = 'local'
    this.log(`Working in ${join(PWD, WORKING_FOLDER)}`)
    if (!existsSync(WORKING_FOLDER)) {
      mkdirSync(WORKING_FOLDER)
    }
    this.log(ref)
    spawnSync('git', [
      'clone',
      '--branch',
      ref,
      'https://github.com/iota-community/iota-wiki.git',
    ], {
      cwd: join(PWD, WORKING_FOLDER),
      shell: true,
      stdio: 'inherit',
    })
    const WIKI_GIT_FOLDER = join(join(PWD, WORKING_FOLDER), readdirSync(join(PWD, WORKING_FOLDER))[0])
    const WIKI_EXTERNAL_FOLDER = join(WIKI_GIT_FOLDER, 'external')
    if (!existsSync(WIKI_EXTERNAL_FOLDER)) {
      mkdirSync(WIKI_EXTERNAL_FOLDER)
    }
    const WIKI_CONTENT_REPO_FOLDER = join(WIKI_EXTERNAL_FOLDER, 'identity.rs')
    if (!existsSync(WIKI_CONTENT_REPO_FOLDER)) {
      mkdirSync(WIKI_CONTENT_REPO_FOLDER)
    }
    const WIKI_CONTENT_DOCS_FOLDER = join(WIKI_CONTENT_REPO_FOLDER, 'documentation')
    if (!existsSync(WIKI_CONTENT_DOCS_FOLDER)) {
      mkdirSync(WIKI_CONTENT_DOCS_FOLDER)
    }
    ['blog', 'docs', 'static', 'sidebars.js'].forEach(folder => {
      const target = join(PWD, folder)
      const path = join(WIKI_CONTENT_DOCS_FOLDER, folder)
      this.log(`linking ${target} to ${path}`)
      symlinkSync(target, path)
    })
    spawnSync('npm', [
      'i',
    ], {
      cwd: WIKI_CONTENT_REPO_FOLDER,
      shell: true,
      stdio: 'inherit',
    })
  }
}
