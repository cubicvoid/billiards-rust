mod command_apex_set;

use std::fmt;
use std::error::Error;
//use std::boxed::Box;


pub fn run(args: &[String]) -> Result<()> {
  println!("hello {:?}", args);
  match args.first() {
    None => Err(CommandLineError::new(&format!("Expected command"))),
    Some(command) => {
      match &command[..] {
        "apexSet" => {
          println!("apexSet");
          command_apex_set::run(&args[1..])
        },
        "pathSet" => {
          println!("pathSet");
          Ok(())
        },
        _ => {
          Err(CommandLineError::new(&format!("Unknown command '{}'", command)))
        },
      }
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    let result = run(&[]);
    result.expect_err("run with empty arguments should yield an error");
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