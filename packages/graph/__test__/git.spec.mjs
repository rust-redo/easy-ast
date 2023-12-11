import test from 'ava'
import { Parser } from '../index.js'
import { getGitRepo, getGitRepoFiles, validateParsed } from './utils.mjs'

test('axios', t => {  
  const parser = new Parser({root: getGitRepo('axios')})
  parser.visit('lib/axios.js', {depth: 3})
  const data = parser.parse()
  const files = getGitRepoFiles('axios', 'lib/**/*.js', [
    '**/null.js', 
    '**/deprecatedMethod.js',
    "**/env/classes/FormData.js",
    "**/platform/browser/**",
  ])
  validateParsed(t, data, files)
})

test('rxjs', t => {  
  const parser = new Parser({root: getGitRepo('rxjs')})
  parser.visit('src/index.ts', {depth: 3})
  const data = parser.parse()
  const files = getGitRepoFiles('rxjs', 'src/**/*.ts', [
  "**/{webSocket,ajax,fetch,operators,testing}/index.ts",
  "**/internal/{ajax,testing}/**",
  "src/internal/operators/partition.ts",
  "src/internal/util/workarounds.ts",
  "src/internal/observable/dom/{fetch,WebSocketSubject,webSocket}.ts"
  ])
  validateParsed(t, data, files)
})

test('nextui', t => {  
  const parser = new Parser({root: getGitRepo('nextui/packages/components')})
  parser.visit('**/src/index.ts', {depth: 3})
  const data = parser.parse()
  const files = getGitRepoFiles('nextui/packages/components', '**/src/**/*.{ts,tsx}', [
  ])
  validateParsed(t, data, files)
})

test('antd', t => {
  const parser = new Parser({ root: getGitRepo('antd') })
  parser.visit('components/index.ts', { depth: 3 })
  const data = parser.parse()
  const files = getGitRepoFiles(
    'antd',
    'components/**/*.{ts,tsx}',
    [
      "**/{__tests__,demo,design}/**",
      "components/icon/**",
      "components/locale/{a,b,ca,cs,d,el,es,et,eu,en_GB,f,g,h,id,is,it,j,k,l,m,n,p,r,ur,uk,s,t,v,x,y,w,z}*",
      "components/{time-picker,date-picker,calendar}/locale/{a,b,c,d,el,es,et,eu,en_GB,f,g,h,i,j,k,l,m,n,p,r,u,u,s,t,v,x,y,w,z}*",
      "components/{qrcode,col/style,row/style}/**",
      "components/statistic/interface*",
    ])
  validateParsed(t, data, files)
})