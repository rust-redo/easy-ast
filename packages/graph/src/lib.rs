#![deny(clippy::all)]

use easy_ast_error::EasyAstError;
use easy_ast_graph_parser::GraphParser;
use easy_ast_napi::{compute_alias, compute_root};
use easy_ast_visitor::Visitor;
use napi::{bindgen_prelude::Buffer, Error, JsError, Status};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Parser {
  parser: Option<GraphParser>,
  visitor: Visitor,
}

#[napi]
impl Parser {
  #[napi(constructor)]
  pub fn new(root: Option<Buffer>, alias: Option<Buffer>) -> Self {
    let root = compute_root(root);
    let alias = compute_alias(&root, alias);

    Self {
      parser: None,
      visitor: Visitor::new(root, alias),
    }
  }

  #[napi]
  pub fn visit(
    &mut self,
    files: Buffer,
    depth: Option<u8>,
    should_resolve: Option<bool>,
  ) -> Result<(), Error> {
    let files = String::from_utf8_lossy(&files).to_string();
    let files = files.split(",").collect();
    let visit_result = self.visitor.visit(files, depth, should_resolve);

    match visit_result {
      Ok(_) => {
        self.parser = Some(GraphParser::new(visit_result.unwrap()));
        Ok(())
      }
      Err(err) => Err(Error::new(
        Status::GenericFailure,
        format!("EASY_AST_ERR: {}", err.unwrap()),
      )),
    }
  }

  #[napi]
  pub fn parse(&self) -> Buffer {
    match self.parser {
      Some(ref p) => serde_json::to_string(p.parse()).unwrap().as_bytes().into(),
      None => Buffer::from("".as_bytes()),
    }
  }
}
