//mod command_apex_set;

use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use clap::{Arg, ArgMatches, App, SubCommand};

use data::apex_set;

pub fn run() {
  let cmd = command();
  let matches = cmd.get_matches();

  root_run(&matches);
}

fn subcommand_apexset_create<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("create")
    .about("Creates a new random apex set")
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
    .about("Lists all apex sets")
}

fn subcommand_apexset_delete<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("delete")
    .about("Deletes an apex set")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the apex set to delete.")
    )
}

fn subcommand_apexset<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("apexset")
    .about("Manipulates sets of random apexes")
    .subcommands(vec![
      subcommand_apexset_create(),
      subcommand_apexset_list(),
      subcommand_apexset_delete(),
    ])
}

fn command<'a, 'b>() -> App<'a, 'b> {
  App::new("billiards-rs")
    .version("0.0.x")
    .subcommand(subcommand_apexset())
}

fn root_run(matches: &ArgMatches) {
  match matches.subcommand() {
    ("apexset", Some(sub_m)) => { apexset_run(sub_m) },
    _ => { println!("{}", matches.usage()); }
  }
}

fn apexset_run(matches: &ArgMatches) {
  let apex_set_manager = {
    let mut path: PathBuf = env::current_dir().unwrap();//?;
    path.push("data");
    path.push("apex_set");
    apex_set::manager(path)
  };
  match matches.subcommand() {
    ("create", Some(sub_m)) => { apexset_create_run(&apex_set_manager, sub_m) },
    ("list", Some(sub_m)) => { apexset_list_run(&apex_set_manager, sub_m) },
    ("delete", Some(sub_m)) => { apexset_delete_run(&apex_set_manager, sub_m) },
    _ => { println!("{}", matches.usage()); }
  }
}

fn apexset_create_run(manager: &apex_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  let count = matches.value_of("count").unwrap().parse::<u32>().unwrap();
  let grid_density = matches.value_of("grid_density").map(|s| s.parse::<u32>().unwrap()).unwrap_or(32);
  let overwrite = matches.is_present("overwrite");
  println!("creating apex set '{}' with count {} and density {}, overwrite: {}", name, count, grid_density, overwrite);
  let apex_set = manager.save(name, overwrite,
    apex_set::random_from_grid(grid_density, count));
  println!("apexSet create: {:?}", apex_set);
}

fn apexset_list_run(manager: &apex_set::Manager, matches: &ArgMatches) {
  println!("listing apex sets");
}

fn apexset_delete_run(manager: &apex_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  println!("deleting apex set '{}'", name);
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