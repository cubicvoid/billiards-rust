use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use chrono::prelude::*;
use clap::{Arg, ArgMatches, App, SubCommand};
use colored::*;

use data::point_set;

pub fn run() {
  let cmd = command();
  let matches = cmd.get_matches();

  root_run(&matches);
}

fn subcommand_pointset_create<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("create")
    .about("Creates a new random point set")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the new point set.")
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
      .help("The number of random points to generate.")
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

fn subcommand_pointset_list<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("list")
    .about("Lists all point sets")
}

fn subcommand_pointset_delete<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("delete")
    .about("Deletes a point set")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the point set to delete.")
    )
}

fn subcommand_pointset<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("pointset")
    .about("Manipulates sets of random points")
    .subcommands(vec![
      subcommand_pointset_create(),
      subcommand_pointset_list(),
      subcommand_pointset_delete(),
    ])
}

fn command<'a, 'b>() -> App<'a, 'b> {
  App::new("billiards-rs")
    .version("0.0.x")
    .subcommand(subcommand_pointset())
}

fn root_run(matches: &ArgMatches) {
  match matches.subcommand() {
    ("pointset", Some(sub_m)) => { pointset_run(sub_m) },
    _ => { println!("{}", matches.usage()); }
  }
}

fn pointset_run(matches: &ArgMatches) {
  let point_set_manager = {
    let mut path: PathBuf = env::current_dir().unwrap();//?;
    path.push("data");
    path.push("point_set");
    point_set::manager(path)
  };
  match matches.subcommand() {
    ("create", Some(sub_m)) => { pointset_create_run(&point_set_manager, sub_m) },
    ("list", Some(sub_m)) => { pointset_list_run(&point_set_manager, sub_m) },
    ("delete", Some(sub_m)) => { pointset_delete_run(&point_set_manager, sub_m) },
    _ => { println!("{}", matches.usage()); }
  }
}

fn pointset_create_run(manager: &point_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  let count = matches.value_of("count").unwrap().parse::<u32>().unwrap();
  let grid_density = matches.value_of("grid_density").map(|s| s.parse::<u32>().unwrap()).unwrap_or(32);
  let overwrite = matches.is_present("overwrite");
  let point_generator = point_set::random_from_grid(grid_density, count);
  if let Err(e) = manager.save(name, overwrite, point_generator) {
    println!("couldn't create point set: {}", e);
  } else {
    println!("saved {} points as '{}'", count, name);
  }
}

struct Tabulator {
  header: Vec<String>,
  rows: Vec<Vec<String>>,
  column_widths: Vec<usize>,
}

impl Tabulator {
  fn new(header: Vec<String>) -> Tabulator {
    let column_widths = header.iter().map(|s| s.len()).collect();
    return Tabulator{header, column_widths, rows: Vec::new()}
  }

  fn append(&mut self, row: Vec<String>) {
    for i in 0..self.header.len() {
      if let Some(s) = row.get(i) {
        let len = s.len();
        if len > self.column_widths[i] {
          self.column_widths[i] = len;
        }
      }
    }
    self.rows.push(row);
  }

  fn display(&self) {
    let text_len: usize = self.column_widths.iter().sum();
    let margin_len = 3 * self.column_widths.len() - 1;
    let top = std::iter::repeat("_").take(text_len + margin_len).collect::<String>();
    println!("+{}+", top);
    self.display_row(&self.header);
    self.display_blank_row();
    for row in &self.rows {
      self.display_row(row);
    }
    let bottom = std::iter::repeat("-").take(text_len + margin_len).collect::<String>();
    println!("+{}+", bottom);
  }

  fn display_row(&self, row: &Vec<String>) {
    let mut result = "|".to_string();
    for (i, s) in row.iter().enumerate() {
      result.push_str(&format!(" {} ", s.cyan()));
      let padding_len = self.column_widths[i] - s.len();
      let padding = std::iter::repeat(" ").take(padding_len).collect::<String>();
      result.push_str(&padding);
      result.push_str("|");
    }
    println!("{}", result);
  }

  fn display_blank_row(&self) {
    let mut result = "|".to_string();
    for width in &self.column_widths {
      let padding_len = width + 2;
      let padding = std::iter::repeat("-").take(padding_len).collect::<String>();
      result.push_str(&padding);
      result.push_str("|");
    }
    println!("{}", result);
  }
}

fn pointset_list_run(manager: &point_set::Manager, matches: &ArgMatches) {
  let point_sets = manager.list().unwrap();
  /*for info in point_sets {
    // the datetime formatting ignores our requested format, i wonder why
    let created = info.created.to_rfc2822().to_string();
    println!("{} {}", info.name, info.created);
  }*/
  let mut table = Tabulator::new(vec![
    String::from("name"),
    String::from("count"),
    String::from("created")]);
  for info in point_sets {
    let count = format!("{}", info.count);
    let created = info.created.to_rfc2822().to_string();
    table.append(vec![info.name.to_string(), count, created]);
  }
  table.display();
  //println!("|{:80}|", "format works as expected. This will be padded".bright_blue())
}

fn pointset_delete_run(manager: &point_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  println!("deleting point set '{}'", name);
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