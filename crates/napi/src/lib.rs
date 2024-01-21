#![deny(clippy::all)]

use std::path::PathBuf;

use easy_ast_resolver::{Alias, AliasValue, ModuleResolver};
use napi::bindgen_prelude::Buffer;

pub fn compute_root(root: Option<Buffer>) -> Option<PathBuf> {
  match root {
    Some(buf) => Some(PathBuf::from(String::from_utf8_lossy(&buf).to_string())),
    _ => None,
  }
}

// alias: name1=path1 name2=path2
pub fn compute_alias(root: &Option<PathBuf>, alias: Option<Buffer>) -> Option<Alias> {
  match alias {
    Some(buf) => {
      let alias_str = String::from_utf8_lossy(&buf).to_string();
      let alias: Alias = alias_str
        .trim()
        .split(" ")
        .map(|s| {
          let kv: Vec<&str> = s.split("=").collect();
          let paths: Vec<AliasValue> = kv[1]
            .split(",")
            .map(|p| {
              AliasValue::Path(root.as_ref().unwrap().join(p).to_str().unwrap().to_string())
            })
            .collect();
          return (kv[0].to_owned(), paths);
        })
        .collect();
      Some(alias)
    }
    _ => None,
  }
}
