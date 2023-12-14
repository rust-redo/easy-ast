import test from 'ava'
import {parser, readParsedFile} from './utils.mjs'

test('should parse export', (t) => {
  parser.visit('./export.ts')
  const parsed = parser.parse('./export.ts')
  const json = readParsedFile(
    'export.json', 
    {},
  )
  const foo = 'nested/foo.js'
  t.truthy(parsed[foo].importer.includes('export.ts'))
  t.truthy(parsed[foo].importer.includes('nested/bar.js'))

  delete parsed[foo].importer
  delete json[foo].importer

  t.deepEqual(
    parsed,
    json 
  )
})