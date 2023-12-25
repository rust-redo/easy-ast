use std::sync::Arc;

use swc_parser::source_map::SourceMap;

pub struct VisitorContext {
  pub process_id: Arc<String>,
  pub source_map: Arc<SourceMap>
}

pub trait BaseVisitor {
  fn set_ctx(&mut self, ctx: VisitorContext);
}