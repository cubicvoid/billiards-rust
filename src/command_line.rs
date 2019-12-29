mod pointset;

use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use chrono::prelude::*;
use clap::{Arg, ArgMatches, App, SubCommand};

pub fn run(root_path: &PathBuf) {
  let cmd = command();
  let matches = cmd.get_matches();

  root_run(root_path, &matches);
}

fn command<'a, 'b>() -> App<'a, 'b> {
  App::new("billiards-rs")
    .version("0.0.x")
    .subcommand(pointset::subcommand())
}

fn root_run(root_path: &PathBuf, matches: &ArgMatches) {
  match matches.subcommand() {
    ("pointset", Some(sub_m)) => { pointset::run(root_path, sub_m) },
    _ => { eprintln!("{}", matches.usage()); }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    //let result = run(&[]);
    //result.expect_err("run with empty arguments should yield an error");
  }
}

#[derive(Debug)]
pub enum CommandLineError {
  IOError(std::io::Error),
  BadCommand(String),
}

impl CommandLineError {
  fn new(msg: &str) -> CommandLineError {
    CommandLineError::BadCommand(msg.to_string())
  }
}

impl fmt::Display for CommandLineError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      CommandLineError::IOError(e) => e.fmt(f),
      CommandLineError::BadCommand(s) => write!(f, "{}", s)
    }
  }
}

impl Error for CommandLineError {
  fn description(&self) -> &str {
    match &self {
      CommandLineError::IOError(e) => e.description(),
      CommandLineError::BadCommand(s) => s
    }
  }
}

impl From<std::io::Error> for CommandLineError {
  fn from(e: std::io::Error) -> Self {
      CommandLineError::IOError(e)
  }
}

pub type Result<T> = std::result::Result<T, CommandLineError>;