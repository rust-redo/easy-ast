import test from 'ava'
import {createParser, readParsedFile} from './utils.mjs'


test('should parse alias path', async (t) => {
  const parser = createParser({'@nested': './nested'})

  await parser.visit('./alias.js')

  t.deepEqual(parser.parse(), readParsedFile('./alias.json'))
})