#![deny(clippy::all)]

use std::str::Split;

use easy_ast_resolver::{Alias, AliasValue, ModuleResolver};
use napi::bindgen_prelude::Buffer;

pub fn compute_root(root: Option<Buffer>) -> Option<String> {
  match root {
    Some(buf) => Some(String::from_utf8_lossy(&buf).to_string()),
    _ => None,
  }
}

// alias: module1:path1 module2:path2
pub fn compute_alias(root: &Option<String>, alias: Option<Buffer>) -> Option<Alias> {
  match alias {
    Some(buf) => {
      let alias_str = String::from_utf8_lossy(&buf).to_string();
      let alias: Alias = alias_str
        .split(" ")
        .map(|s| {
          let kv: Vec<&str> = s.split(":").collect();
          let paths: Vec<AliasValue> = kv[1]
            .split(",")
            .map(|p| {
              return AliasValue::Path(ModuleResolver::resolve_file(root.as_ref().unwrap(), p));
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
