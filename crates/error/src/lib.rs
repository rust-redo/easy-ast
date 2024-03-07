#[derive(Debug)]
pub enum EasyAstError {
  FileNotFound(String),
  SyntaxError(String),
  InvalidExtension(String),
}

impl EasyAstError {
  pub fn unwrap(self) -> String {
    match self {
      Self::FileNotFound(msg) | Self::SyntaxError(msg) | Self::InvalidExtension(msg) => msg,
    }
  }
}
