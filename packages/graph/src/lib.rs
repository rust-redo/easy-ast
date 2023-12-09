#![deny(clippy::all)]

use easy_ast_napi::{compute_alias, compute_root};
use easy_ast_parser::Parser as InternalParser;
use napi::bindgen_prelude::Buffer;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Parser {
  parser: InternalParser,
}

#[napi]
impl Parser {
  #[napi(constructor)]
  pub fn new(root: Option<Buffer>, alias: Option<Buffer>) -> Self {
    let root = compute_root(root);
    let alias = compute_alias(&root, alias);
    Self {
      parser: InternalParser::new(root, alias),
    }
  }

  #[napi]
  pub fn parse(&self, files: Buffer, depth: Option<u8>, should_resolve: Option<bool>) -> Buffer {
    let files = String::from_utf8_lossy(&files).to_string();
    let files = files.split(",").collect();

    serde_json::to_string(&self.parser.parse(files, depth, should_resolve))
      .unwrap()
      .as_bytes()
      .into()
  }
}
