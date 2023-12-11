#[derive(Debug)]
pub enum EasyAstError {
  FileNotFound(String),
  SyntaxError(String),
}

impl EasyAstError {
  pub fn unwrap(self) -> String {
    match self {
      Self::FileNotFound(msg) => msg,
      Self::SyntaxError(msg) => msg,
    }
  }
}
