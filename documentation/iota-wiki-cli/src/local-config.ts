import {existsSync} from 'fs'
import {readJSON} from 'fs-extra'
import {join} from 'path'

export async function getLocalConfig(this: any) {
  const PWD = process.env.PWD ?? ''
  const configPath = join(PWD, 'config.json')
  if (!existsSync(configPath)) {
    this.error('local config.json not found')
  }
  const userConfig = await readJSON(configPath)
  return userConfig
}
