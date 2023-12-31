use easy_ast_error::EasyAstError;
use std::{path::Path, sync::Arc};
use swc_core::{
  base::{config::IsModule, Compiler},
  common::{
    errors::{ColorConfig, Handler},
    Globals, Mark, SourceMap, GLOBALS,
  },
  ecma::{
    ast::EsVersion,
    parser::Syntax,
    transforms::base::resolver,
    visit::{VisitMutWith, VisitWith},
  },
};
use swc_ecmascript::{
  parser::{EsConfig, TsConfig},
  visit::Visit,
};

pub use swc_ecmascript::*;

pub struct SwcParser {
  source_map: Arc<SourceMap>,
  handler: Handler,
  compiler: Compiler,
}

impl SwcParser {
  pub fn new() -> SwcParser {
    let source_map = Arc::<SourceMap>::default();
    SwcParser {
      source_map: source_map.clone(),
      handler: Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone())),
      compiler: swc::Compiler::new(source_map.clone()),
    }
  }

  pub fn run<F, R>(&self, f: F) -> R
  where
    F: FnOnce() -> R,
  {
    GLOBALS.set(&Globals::new(), f)
  }

  /// parse single js file
  pub fn parse_file(&self, file: &str, visitor: &mut dyn Visit) -> Result<(), EasyAstError> {
    let (syntax, is_js, is_ts) = self.get_options(file);

    if !is_js {
      return Ok(());
    }

    let fm_result = self.source_map.load_file(Path::new(file));

    if fm_result.is_err() {
      return Err(EasyAstError::FileNotFound(format!(
        "failed to load {}",
        file
      )));
    }

    let fm = fm_result.unwrap();

    // XXX: syntax error is unrecoverable
    let program_result = self.compiler.parse_js(
      fm,
      &self.handler,
      EsVersion::latest(),
      syntax,
      IsModule::Unknown,
      None,
    );

    if program_result.is_err() {
      return Err(EasyAstError::SyntaxError(format!("{} syntax error", file)));
    }

    let mut program = program_result.unwrap();

    program.visit_mut_with(&mut resolver(Mark::new(), Mark::new(), is_ts));
    program.visit_with(visitor);
    Ok(())
  }

  /// return (Syntax, is_js, is_ts)
  fn get_options(&self, file: &str) -> (Syntax, bool, bool) {
    if file.ends_with(".ts") {
      return (Syntax::Typescript(Default::default()), true, true);
    }

    if file.ends_with(".tsx") {
      return (
        Syntax::Typescript(TsConfig {
          tsx: true,
          ..Default::default()
        }),
        true,
        true,
      );
    }

    if file.ends_with(".jsx") {
      return (
        Syntax::Es(EsConfig {
          jsx: true,
          ..Default::default()
        }),
        true,
        false,
      );
    }

    if file.ends_with(".js") {
      return (Syntax::default(), true, false);
    }

    return (Syntax::default(), false, false);
  }
}
