// import test from 'ava'
import {parser, readParsedFile} from './utils.mjs'

console.log(JSON.stringify(parser.parse('./export.ts')))
// test('should parse export', (t) => {
//   t.deepEqual(
//     readParsedFile(
//       'es.json', 
//       {'es.js': 'es.js'},
//     )
//   )
// })