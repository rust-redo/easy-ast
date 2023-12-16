import {join} from 'node:path'
import {spawn} from 'node:child_process'
import fg from 'fast-glob'

const bindings = [
  'bindings-aarch64-apple-darwin',
  'bindings-aarch64-linux-android',
  'bindings-aarch64-pc-windows-msvc',
  'bindings-aarch64-unknown-linux-gnu',
  'bindings-aarch64-unknown-linux-musl',
  'bindings-armv7-linux-androideabi',
  'bindings-armv7-unknown-linux-gnueabihf',
  'bindings-freebsd',
  'bindings-i686-pc-windows-msvc',
  'bindings-universal-apple-darwin',
  'bindings-x86_64-apple-darwin',
  'bindings-x86_64-pc-windows-msvc',
  'bindings-x86_64-unknown-linux-gnu',
  'bindings-x86_64-unknown-linux-musl'
]

bindings.forEach(async (dir, index) => {
  // move binaries
  const bins = await fg('**/*.node', {cwd: join(dir, './packages')})
  let cmd = ``
  bins.forEach(bin => {
    const [pkg, file] = bin.split('/')
    cmd = [cmd, `mv ${join(dir, 'packages', bin)} ${join('packages', pkg, dir, file)}`].join('&&')
  })

  spawn(cmd, {stdio: 'inherit'})

  // move js
  if(index === 0) {
    const jss = await fg('**/core.*', {cwd: join(dir, './packages')})
    cmd = ''
    jss.forEach(js => {
      const [pkg, file] = js.split('/')
      cmd = [cmd, `mv ${join(dir, 'packages', js)} ${join('packages', pkg, file)}`].join('&&')
    })

    spawn(cmd, {stdio: 'inherit'})
  }
})