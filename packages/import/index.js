"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Parser = exports.ImportLinkType = exports.ImportNodeType = void 0;
const node_path_1 = require("node:path");
const fast_glob_1 = __importDefault(require("fast-glob"));
const core_1 = require("./core");
var ImportNodeType;
(function (ImportNodeType) {
    ImportNodeType["BUILTIN"] = "builtin";
    ImportNodeType["INTERNAL"] = "internal";
    ImportNodeType["EXTERNAL"] = "external";
})(ImportNodeType || (exports.ImportNodeType = ImportNodeType = {}));
var ImportLinkType;
(function (ImportLinkType) {
    ImportLinkType["STATIC"] = "static";
    ImportLinkType["DYNAMIC"] = "dynamic";
    ImportLinkType["REQUIRE"] = "require";
})(ImportLinkType || (exports.ImportLinkType = ImportLinkType = {}));
class Parser {
    constructor({ root = './', alias } = {}) {
        const absRoot = (0, node_path_1.isAbsolute)(root) ? root : (0, node_path_1.resolve)(process.cwd(), root);
        this.root = root;
        this.parser = new core_1.Parser(Buffer.from(absRoot), alias ? Buffer.from(alias) : undefined);
    }
    parse(files, { depth, resolve, buffer, } = {}) {
        const fileArr = (Array.isArray(files) ? files : [files]).reduce((acc, file) => {
            if (fast_glob_1.default.isDynamicPattern(file)) {
                acc.push(...fast_glob_1.default.sync(file, { cwd: this.root }));
            }
            else {
                acc.push(file);
            }
            return acc;
        }, []);
        const parsed = this.parser.parse(Buffer.from(fileArr.toString()), depth, resolve);
        return buffer ? parsed : JSON.parse(parsed.toString());
    }
}
exports.Parser = Parser;
