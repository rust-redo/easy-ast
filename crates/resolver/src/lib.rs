use std::{
  path::{Path, PathBuf},
  sync::Arc,
};

use easy_ast_error::EasyAstError;
use oxc_resolver::{ResolveError, ResolveOptions, Resolver};

pub use oxc_resolver::{Alias, AliasValue};

pub struct ModuleResolver {
  resolver: Resolver,
  root: Arc<PathBuf>,
  pub should_resolve: bool,
}

impl ModuleResolver {
  pub fn new(root: Arc<PathBuf>, should_resolve: bool, alias: Arc<Alias>) -> Self {
    Self {
      root,
      should_resolve,
      resolver: Resolver::new(ResolveOptions {
        builtin_modules: true,
        alias: alias.to_vec(),
        extensions: vec![
          ".js".to_string(),
          ".ts".to_string(),
          ".jsx".to_string(),
          ".tsx".to_string(),
        ],
        ..ResolveOptions::default()
      }),
    }
  }

  /// return file absolute path based on source
  pub fn resolve_file(source: &PathBuf, file: &str) -> Result<String, EasyAstError> {
    let result = Path::new(source).join(Path::new(file)).canonicalize();

    match result {
      Ok(buf) => Ok(buf.to_str().unwrap().to_string()),
      Err(err) => Err(EasyAstError::FileNotFound(format!(
        "failed to resolve {} from {}: {}",
        file,
        source.to_string_lossy(),
        err
      ))),
    }
  }

  /// return (relative_path, in_root)
  pub fn resolve_relative_root(&self, file: &str) -> (String, bool) {
    let path_buf = Path::new(file);
    println!("{}\n{}", self.root.to_string_lossy(), file);
    if path_buf.starts_with(self.root.as_ref()) {
      let mut path_buf = path_buf.strip_prefix(self.root.as_ref()).unwrap();

      if path_buf.starts_with("/") {
        path_buf = path_buf.strip_prefix("/").unwrap();
      }

      return (path_buf.to_string_lossy().to_string(), true);
    }

    return (file.replace("./", ""), file.starts_with("."));
  }

  /// return module absolute path based on source
  pub fn resolve_module(
    &self,
    source: &str,
    request: &str,
  ) -> Result<(String, bool), EasyAstError> {
    let source_dir = &ModuleResolver::resolve_file(&self.root, source)?;
    let source_dir = Path::new(source_dir)
      .parent()
      .unwrap_or_else(|| Path::new("/"));
    let id = if self.should_resolve {
      match self.resolver.resolve(source_dir, request) {
        Ok(res) => res.full_path().to_string_lossy().to_string(),
        Err(err) => match err {
          // builtin module
          ResolveError::Builtin(file_name) => file_name,
          _ => request.to_owned(),
        },
      }
    } else {
      request.to_owned()
    };

    Ok(self.resolve_relative_root(&id))
  }
}
