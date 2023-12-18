import test from 'ava'
import {createParser, readParsedFile} from './utils.mjs'


test('should parse alias path', async (t) => {
  const parser = createParser({'@nested': 'nested'})

  await parser.visit('./alias.js')
  const parsed = parser.parse()
  const expected = readParsedFile('./alias.json')
  const foo = 'nested/foo.js'
  t.truthy(parsed[foo].importer.includes('nested/bar.js'))
  t.truthy(parsed[foo].importer.includes('alias.js'))
  delete parsed[foo].importer
  delete expected[foo].importer
  t.deepEqual(parsed, expected) 
})