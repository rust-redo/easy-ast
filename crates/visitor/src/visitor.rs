use easy_ast_error::EasyAstError;
use easy_ast_resolver::{Alias, ModuleResolver};
use easy_ast_visitor_import::{ImportNode, ImportNodeKind, ImportVisitor};
use std::{collections::HashMap, env, sync::Arc, path::{PathBuf, Path}};
use swc_parser::SwcParser;

pub struct Visitor {
  swc: SwcParser,
  root: Arc<PathBuf>,
  alias: Arc<Alias>,
}

impl Visitor {
  pub fn new(root: Option<PathBuf>, alias: Option<Alias>) -> Visitor {
    Visitor {
      swc: SwcParser::new(),
      root: Arc::new(match root {
        Some(r) => r,
        _ => env::current_dir().unwrap(),
      }),
      alias: Arc::new(alias.unwrap_or(vec![])),
    }
  }

  pub fn visit(
    &self,
    files: Vec<&str>,
    depth: Option<u8>,
    should_resolve: Option<bool>,
  ) -> Result<ImportVisitor, EasyAstError> {
    let wrapped_depth = depth.unwrap_or(2);
    let wrapped_should_resolve = should_resolve.unwrap_or(true);
    let mut visitor: ImportVisitor = ImportVisitor::new(ModuleResolver::new(
      self.root.clone(),
      wrapped_should_resolve,
      self.alias.clone(),
    ));

    self.swc.run(|| {
      let mut processed_ids: HashMap<Arc<String>, bool> = HashMap::new();

      for file in files.iter() {
        self.deep_visit(
          file,
          &mut visitor,
          if wrapped_should_resolve {
            wrapped_depth
          } else {
            1
          },
          &mut processed_ids,
        )?;
      }

      Ok(visitor)
    })
  }

  fn deep_visit<'a>(
    &self,
    file: &str,
    visitor: &mut ImportVisitor,
    mut depth: u8,
    processed_ids: &mut HashMap<Arc<String>, bool>,
  ) -> Result<(), EasyAstError> {
    let mut file_queue = vec![Arc::new(file.to_owned())];
    let mut current_count = 1;
    let mut next_count = 0;

    while file_queue.is_empty() == false && depth > 0 {
      let target_file = file_queue.pop().unwrap();
      let resolved_file = Arc::new(ModuleResolver::resolve_file(&self.root, &target_file)?);
      let process_id = Arc::new(visitor.resolver.resolve_relative_root(&target_file).0);

      if processed_ids.contains_key(&process_id.clone()) == false {
        processed_ids.insert(process_id.clone(), true);

        visitor.set_process_id(process_id.clone());
        visitor.create_node(process_id.clone());
        self.swc.parse_file(&resolved_file, visitor);

        for (id, node) in &visitor.import_node.map {
          if processed_ids.contains_key(&id.clone()) || node.kind != ImportNodeKind::Internal {
            continue;
          }
          next_count += 1;
          file_queue.push(id.clone());
        }
      }

      current_count -= 1;

      if current_count == 0 {
        current_count = next_count;
        depth -= 1;
      }
    }

    Ok(())
  }
}
