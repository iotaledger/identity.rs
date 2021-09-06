import {Command, flags} from '@oclif/command'
import {join} from 'path'
import {rmdirSync} from 'fs'

export default class Nuke extends Command {
  static description = 'completely removes local wiki'

  static flags = {
    help: flags.help({char: 'h'}),
    // flag with a value (-n, --name=VALUE)
  }

  async run() {
    const PWD = process.env.PWD ?? ''
    const LOCAL_WIKI_FOLDER = join(PWD, 'local')
    rmdirSync(LOCAL_WIKI_FOLDER, {recursive: true})
  }
}
