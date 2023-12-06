use std::sync::Arc;

use swc_parser::{
  ast::{ExportAll, ImportDecl, ImportSpecifier as SwcImportSpecifier, ExportNamedSpecifier, ModuleDecl, NamedExport},
  visit::{noop_visit_type, Visit},
};

use crate::{
  node::{self, ImportLink, ImportLinkKind, ImportNode, ImportNodeKind, ImportNodeMap, ImportSpecifier},
  resolver::ImportResolver,
};

pub struct ImportVisitor {
  pub import_node: ImportNodeMap,
  process_id: Option<Arc<String>>,
  pub resolver: ImportResolver,
}

impl ImportVisitor {
  pub fn new(resolver: ImportResolver) -> Self {
    Self {
      import_node: ImportNodeMap::new(),
      process_id: None,
      resolver,
    }
  }

  pub fn set_process_id(&mut self, id: Arc<String>) {
    self.process_id = Some(id.clone());
  }

  /// insert node if not exist
  pub fn create_node(&mut self, id: Arc<String>) {
    if self.import_node.map.get_mut(&id).is_none() {
      self.import_node.create_node(&id);
    }
  }

  fn resolve_from_process_id(&self, request: &str) -> ImportNode {
    let (id, in_root) = self
      .resolver
      .resolve_module(self.process_id.as_ref().unwrap(), request);

    ImportNode {
      kind: ImportNodeKind::compute(&id, in_root),
      id: Arc::new(id),
      ..ImportNode::default()
    }
  }

  fn insert_process_node_depent(&mut self, src: &[u8]) -> (Arc<String>, &mut ImportNode) {
    let process_id = self.process_id.clone().unwrap();
    let module_node = self.resolve_from_process_id(&String::from_utf8_lossy(src));
    let module_id = module_node.id.clone();
    (
      module_id,
      self.import_node.insert_node_depend(&process_id, module_node),
    )
  }
}

impl Visit for ImportVisitor {
  noop_visit_type!();

  // fn visit_mut_import_named_specifier(&mut self, import: &mut ImportNamedSpecifier) {
  //     dbg!(&import.imported);
  //     dbg!(&import.local);
  //     dbg!(&import.is_type_only);
  // }

  fn visit_import_decl(&mut self, import: &ImportDecl) {
    let (module_id, process_node) = self.insert_process_node_depent(&import.src.value.as_bytes());

    let imports = process_node.import.as_mut().unwrap();
    let mut ident: Vec<ImportSpecifier> = vec![];
    for spec in import.specifiers.iter() {
      match spec {
        SwcImportSpecifier::Named(ref named_spec) => {
          let name = Arc::new(named_spec.local.sym.to_string());
          ident.push(ImportSpecifier {
            name: name.clone(),
            _as: name,
            is_type: named_spec.is_type_only
          });
        }
        SwcImportSpecifier::Default(ref default_spec) => {
          let _as = Arc::new(default_spec.local.sym.to_string());
          ident.push(ImportSpecifier {
            name: Arc::new("default".into()),
            _as,
            is_type: false
          })
        }
        SwcImportSpecifier::Namespace(ref namespace) => ident.push(ImportSpecifier {
          name: Arc::new("*".into()),
          _as: Arc::new(namespace.local.sym.to_string()),
          is_type: false
        }),
      }
    }

    imports.push(ImportLink {
      id: module_id,
      kind: ImportLinkKind::Static,
      ident,
      type_only: import.type_only
    });

    // println!("serde {}", serde_json::to_string(&self.import_node.import).unwrap());
    // dbg!(&import.specifiers);
  }

  fn visit_export_all(&mut self, export: &ExportAll) {
    if export.type_only {
      return;
    }
    let (module_id, process_node) = self.insert_process_node_depent(&export.src.value.as_bytes());
    let imports = process_node.import.as_mut().unwrap();
    let name: Arc<String> = Arc::new("*".into());
    let ident: Vec<ImportSpecifier> = vec![ImportSpecifier {name: name.clone(), _as: name, is_type: false}];

    imports.push(ImportLink {
      id: module_id,
      kind: ImportLinkKind::Static,
      ident,
      type_only: false
    });
  }

  fn visit_named_export(&mut self, export: &NamedExport) {
    if export.type_only {
      return;
    }

    // dbg!(export);
  }
  // fn visit_export_decl(&mut self, export: &ExportDecl){
  //   println!("{:?}", export);
  // }
  // fn visit_mut_ts_import_equals_decl(&mut self, import: &mut TsImportEqualsDecl) {
  //   dbg!(&import.id);
  //   dbg!(&import.module_ref);
  //   dbg!(&import.is_export);
  //   dbg!(&import.is_type_only);
  //   dbg!(&import.span);
  // }
}
