import { isAbsolute, resolve } from 'node:path'
import fg from 'fast-glob'
import {Parser as CoreParser} from './core'

export enum ImportNodeType {
  BUILTIN = 'builtin',
  INTERNAL = 'internal',
  EXTERNAL = 'external'
}
export interface ImportNode {
  id: string
  type: ImportNodeType
  importer: string[] | null
  import: ImportLink[] | null
}

export enum ImportLinkType {
  STATIC = 'static',
  DYNAMIC = 'dynamic',
  REQUIRE = 'require'
}

export interface ImportLink {
  id: string
  type: ImportLinkType
  ident: {name: string, as: string}[]
}


export class Parser {
  parser: CoreParser
  root: string
  constructor({root = './', alias}: {root?: string, alias?: Record<string, string>} = {}) {
    const absRoot = isAbsolute(root) ? root : resolve(process.cwd(), root)
    this.root = root
    this.parser = new CoreParser(
      Buffer.from(absRoot), 
      alias ? Buffer.from(
        Object
          .keys(alias)
          .reduce(
            (s, key) => [s, `${key}=${
              isAbsolute(alias[key]) ? alias[key] : resolve(absRoot, alias[key])
            }`].join(' '), 
            ''
          )
      ) : undefined
    )
  }

  visit(
    files: string | string[], 
    {
      depth,
      resolve,
    }: {
      depth?: number,
      resolve?: boolean
    } = {}
  )  {
    const fileArr = (Array.isArray(files) ? files : [files]).reduce((acc, file) => {
      if(fg.isDynamicPattern(file)) {
        acc.push(...fg.sync(file, {cwd: this.root}))
      } else {
        acc.push(file)
      }
      return acc
    }, [] as string[])

    this.parser.visit(Buffer.from(fileArr.toString()), depth, resolve)
  }

  parse() {
    const parsed = this.parser.parse()
    return JSON.parse(parsed.toString())
  }
}

