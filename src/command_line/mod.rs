mod apex_set;

use std::fmt;
use std::error::Error;
//use std::boxed::Box;


pub fn run(args: &[String]) -> Result<(), CommandLineError> {
  println!("hello {:?}", args);
  match args.first() {
    None => Err(CommandLineError::new(&format!("Expected command"))),
    Some(command) => {
      match &command[..] {
        "apexSet" => {
          println!("apexSet");
          apex_set::run(&args[1..])
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
    run(&[]);
  }
}

#[derive(Debug)]
pub struct CommandLineError {
  details: String
}

impl CommandLineError {
  fn new(msg: &str) -> CommandLineError {
    CommandLineError{details: msg.to_string()}
  }
}

impl fmt::Display for CommandLineError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.details)
  }
}

impl Error for CommandLineError {
  fn description(&self) -> &str {
      &self.details
  }
}
