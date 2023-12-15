import test from 'ava'
import {parser} from './utils.mjs'


test('should parse file not found error', (t) => {
  try {
    parser.visit('./error/not-found.js')
  } catch (err) {
    t.true(/^EASY_AST_ERR.+failed to resolve foo.js from.+/.test(err.message))
  }
})