pub mod point_set;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DataError {
  IOError(std::io::Error),
  JSONError(serde_json::Error),
  OsStringError(std::ffi::OsString),
  Unimplemented(String)
}

impl fmt::Display for DataError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //String::from(self.description()).fmt(f)
    match &self {
      DataError::IOError(e) => e.fmt(f),
      DataError::JSONError(e) => e.fmt(f),
      DataError::OsStringError(e) => e.clone().into_string().expect("invalid unicode").fmt(f),
      DataError::Unimplemented(s) => format!("Unimplemented: {}", s).fmt(f)
    }
  }
}

impl From<std::io::Error> for DataError {
  fn from(e: std::io::Error) -> Self {
    DataError::IOError(e)
  }
}

impl From<serde_json::Error> for DataError {
  fn from(e: serde_json::Error) -> Self {
    DataError::JSONError(e)
  }
}

impl From<std::ffi::OsString> for DataError {
  fn from(e: std::ffi::OsString) -> Self {
    DataError::OsStringError(e)
  }
}

pub type Result<T> = std::result::Result<T, DataError>;

//use crate::geometry::*;