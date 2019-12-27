mod command_apex_set;

use std::fmt;
use std::error::Error;
use clap::{Arg, App, SubCommand};
//use std::boxed::Box;

fn subcommand_apexset_create<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("create")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the new apex set.")
    )
    .arg(Arg::with_name("count")
      .short("c")
      .long("count")
      .takes_value(true)
      .required(true)
      .validator(|count| {
        count.parse::<u32>()
          .map(|_| {})
          .map_err(|_| "expected integer".to_string())
      })
      .help("The number of random apexes to generate.")
    )
    .arg(Arg::with_name("grid_density")
      .short("g")
      .long("grid_density")
      .value_name("density")
      .takes_value(true)
      .validator(|density| {
        density.parse::<u32>()
          .map(|_| {})
          .map_err(|_| "expected integer".to_string())
      })
      .help("The density of the generating grid, in log base 2.")
    )
    .arg(Arg::with_name("overwrite")
      .short("o")
      .long("overwrite")
      .requires("name")
      .help("Overwrite this set if it already exists.")
    )       
}

fn subcommand_apexset_list<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("list")
}

fn subcommand_apexset_delete<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("delete")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the apex set to delete.")
    )
}

fn subcommand_apexset<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("apexset")
    .subcommands(vec![
      subcommand_apexset_create(),
      subcommand_apexset_list(),
      subcommand_apexset_delete(),
    ])
}

pub fn run() {
  let matches = App::new("billiards-rs")
    .version("0.0.x")
    .subcommand(subcommand_apexset())
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("apexset") {
    println!("apexset");
    if let Some(matches) = matches.subcommand_matches("create") {
      println!("create");
      let overwrite = matches.occurrences_of("overwrite");
      println!("overwrite: {}", overwrite > 0);
    }
  }
}

pub fn run_old(args: &[String]) -> Result<()> {
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