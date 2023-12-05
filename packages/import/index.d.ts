/// <reference types="node" />
import { Parser as CoreParser } from './core';
export declare enum ImportNodeType {
    BUILTIN = "builtin",
    LOCAL = "local",
    NODE_MODULES = "node_modules"
}
export interface ImportNode {
    id: string;
    type: ImportNodeType;
    importer: string[] | null;
    import: ImportLink[] | null;
}
export declare enum ImportLinkType {
    STATIC = "static",
    DYNAMIC = "dynamic",
    REQUIRE = "require"
}
export interface ImportLink {
    id: string;
    type: ImportLinkType;
    ident: {
        name: string;
        as: string;
    }[];
}
export declare class Parser {
    parser: CoreParser;
    root: string;
    constructor({ root, alias }?: {
        root?: string;
        alias?: string;
    });
    parse(files: string | string[], { depth, resolve, buffer, }?: {
        depth?: number;
        resolve?: boolean;
        buffer?: boolean;
    }): Record<string, ImportNode> | Buffer;
}
