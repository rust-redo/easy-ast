use easy_ast_visitor_import::{ImportNode, ImportVisitor};
use std::{collections::HashMap, sync::Arc};

pub struct GraphParser {
  visitor: ImportVisitor,
}

impl GraphParser {
  pub fn new(visitor: ImportVisitor) -> Self {
    Self { visitor }
  }

  pub fn parse(&self) -> &HashMap<Arc<String>, ImportNode> {
    &self.visitor.import_node.map
  }
}
