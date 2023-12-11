import test from 'ava'
import { parser, readParsedFile } from './utils.mjs'

test('should parse with resolve and recursion', (t) => {
  const importJson =  readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js',
      'foo.js': 'nested/foo.js',
      'bar.js': 'nested/bar.js',
    },
    { semver: 'semver/index.js' }
  )
  parser.visit('resolve.js', {depth: 3})
  const parsedTree = parser.parse()

  importJson['nested/foo.js'].importer.forEach(
    id => {
      t.true(parsedTree['nested/foo.js'].importer.includes(id))
    }
  )
  
  // importer order is random
  importJson['nested/foo.js'].importer = parsedTree['nested/foo.js'].importer

  t.deepEqual(
    parsedTree,
    importJson
  )
})

test('should parse without recursion', (t) => {
  const importJson =  readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js',
      'foo.js': 'nested/foo.js',
      'bar.js': 'nested/bar.js',
    },
    { semver: 'semver/index.js' }
  )

  importJson['nested/bar.js'].import = null
  importJson['nested/foo.js'].importer.pop()

  parser.visit('resolve.js', {depth: 1})
  t.deepEqual(
    parser.parse(),
   importJson
  )
})

test('should parse without resolve', t => {
  const importJson = readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js',
      'foo.js': 'nested/foo',
      'bar.js': 'nested/bar',
    },
    { semver: 'semver' },
    false
  )

  importJson['nested/bar'].import = null
  importJson['nested/foo'].importer.pop()

  parser.visit('resolve.js', { resolve: false })

  t.deepEqual(
    parser.parse(),
    importJson
  )
})

test('should parse without recursion & resolve', (t) => {
  const importJson = readParsedFile(
    'resolve.json',
    { 
      'resolve.js': 'resolve.js', 
      'foo.js': 'nested/foo',
      'bar.js': 'nested/bar',
    },
    { semver: 'semver' },
    false
  )

  importJson['nested/bar'].import = null
  importJson['nested/foo'].importer.pop()

  parser.visit('resolve.js', { depth: 1, resolve: false })

  t.deepEqual(
    parser.parse(),
    importJson
  )
})