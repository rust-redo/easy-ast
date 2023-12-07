import test from 'ava'
import {parser, readParsedFile} from './utils.mjs'

test('should parse export', (t) => {
  t.deepEqual(
    parser.parse('./export.ts'),
    readParsedFile(
      'export.json', 
      {},
    )
  )
})