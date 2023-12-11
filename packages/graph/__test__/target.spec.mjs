import test from 'ava'
import {parser, readParsedFile} from './utils.mjs'

test('should parse es', (t) => {
  parser.visit('es.js');

  t.deepEqual(
    parser.parse(), 
    readParsedFile(
      'es.json', 
      {'es.js': 'es.js'},
    )
  )
})

test('should parse ts', (t) => {
  parser.visit("es.ts")
  t.deepEqual(parser.parse(), readParsedFile('es-ts.json', {'es.ts': 'es.ts'},))
})

test('should parse jsx', (t) => {
  parser.visit('es.jsx')
  t.deepEqual(
    parser.parse(), 
    readParsedFile(
      'es-jsx.json', 
      {'es.jsx': 'es.jsx'},
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})

test('should parse tsx', (t) => {
  parser.visit('es.tsx')
  t.deepEqual(
    parser.parse(), 
    readParsedFile(
      'es-tsx.json', 
      {'es.tsx': 'es.tsx'},
      {
        semver: 'semver/index.js',
        react: 'react/index.js'
      }
    )
  )
})