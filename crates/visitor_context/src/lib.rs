use std::sync::Arc;

use swc_parser::{source_map::SourceMap, SourceFile, Span};

pub struct VisitorContext {
  pub process_id: Arc<String>,
  pub source_map: Arc<SourceMap>,
  pub source_file: Arc<SourceFile>,
}

impl VisitorContext {
  // return ((start_offset, end_offset), (row, col))
  pub fn lookup_pos(&self, span: Span) -> ((u32, u32), (usize, usize)) {
    let row_col = self
      .source_map
      .lookup_char_pos_with(self.source_file.clone(), span.lo);

    (
      self.source_map.span_to_char_offset(&self.source_file, span),
      (
        row_col.line,
        row_col.col.0
      ),
    )
  }
}
