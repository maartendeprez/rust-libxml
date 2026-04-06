use std::ffi::NulError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("tree error")]
  Tree,
  #[error("parse error")]
  Parser,
  #[error("xpath error")]
  XPath,
  #[error("failed to build c string: {0}")]
  CStr(NulError),
}
