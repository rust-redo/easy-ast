import { mkdir, access, constants, writeFile } from 'node:fs/promises'
import { join } from 'node:path'
import { spawn } from 'node:child_process'
import debug from 'debug'
import { simpleGit, CleanOptions } from 'simple-git'

const repoRootDir = join(process.cwd(), 'repos')
// [[name, origin, branch]...]
const repos = [
  ['axios', 'https://github.com/axios/axios.git', 'v1.6.2'],
  ['rxjs', 'https://github.com/ReactiveX/rxjs.git', '8.0.0-alpha.12'],
  ['nextui', 'https://github.com/nextui-org/nextui.git', 'v2.0.0'],
  ['antd', 'https://github.com/ant-design/ant-design', '5.11.2', async () => {
    // skip version/index.ts error for now
    await writeFile(join(repoRootDir, 'antd/components/version/index.ts'), `export default '1.0.0'`)
  }]
]

async function exist(file) {
  return (await access(file, constants.R_OK | constants.W_OK).catch(() => false)) ?? true
}

process.on('uncaughtException', err => console.log(err))

async function run() {
  debug.enable('simple-git:output*');

  // create ./repos dir
  if (!(await exist(repoRootDir))) {
    await mkdir(repoRootDir)
  }

  await Promise.all(repos.map(async ([name, origin, branch, onFinish]) => {
    const repoDir = join(repoRootDir, name)
    const git = simpleGit({ maxConcurrentProcesses: 6, }).clean(CleanOptions.FORCE)

    try {
      if (!(await exist(repoDir))) {
        console.log(`  [git clone ${name}...]`)
        await git.clone(origin, repoDir)
        await git.cwd(repoDir)
        await git.checkout(branch)
        await onFinish?.()
      }

      console.log(`  [install ${name} dependencies...]`)
      spawn(`pnpm`, ['install'], { stdio: 'ignore', cwd: repoDir })
    } catch (err) {console.log(err) }
  }))
}

run()