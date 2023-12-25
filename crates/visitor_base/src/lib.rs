use std::sync::Arc;

use swc_parser::{source_map::SourceMap, Span, SourceFile};

pub struct VisitorContext {
  pub process_id: Arc<String>,
  pub source_map: Arc<SourceMap>,
  pub source_file: Arc<SourceFile>
}

pub trait BaseVisitor {
  fn set_ctx(&mut self, ctx: VisitorContext);
  // return ((start_offset, end_offset), (start_line, end_line))
  fn lookup_pos(&self, span: Span) -> ((u32, u32), (Option<usize>, Option<usize>)); 
}