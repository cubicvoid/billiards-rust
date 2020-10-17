use std::fmt;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub enum NamespaceEntry {
	PointSet,
	PathSet,
	Point,
	Path,
}

pub struct ReplState {
	pub namespace: HashMap<String, NamespaceEntry>,

	pub cancelled: bool,
}

impl ReplState {
	pub fn handle_line(&mut self, line: &str) {
		let mut words = line.split(char::is_whitespace);

		if let Some(command) = words.next() {
			if command == "plot" {
				self.cmd_plot(words);
			} else if command == "load" {
				self.cmd_load(words);
			}
		}
	}

	fn cmd_plot<'a, WordIter>(&self, mut words: WordIter)
	where
		WordIter: Iterator<Item = &'a str>
	{
		if let Some(source) = words.next() {

			println!("thought u cd plot {}, did u", source);
		} else {
			println!("plot: expected source (e.g. 'plot cerberus[0]')");
		}
	}

	fn cmd_load<'a, WordIter>(&self, mut words: WordIter)
	where
		WordIter: Iterator<Item = &'a str>
	{
		for source_spec in words {
			//let something: Vec<&str> = sourceSpec.split('.').collect();
			//let
			match parse_load_source(source_spec) {
				Ok(source) => println!("loading source {}", source),
				Err(e) => println!("idk about '{}': {}", source_spec, e)
			}
		}
	}
}



enum LoadSource {
	PointSet(String),
	PathSet(String),
}

impl fmt::Display for LoadSource {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			LoadSource::PointSet(name) => write!(f, "PointSet({})", name),
			LoadSource::PathSet(name) => write!(f, "PathSet({})", name),
		}
  }

}

fn parse_load_source(source: &str) -> Result<LoadSource, String> {
	return Ok(LoadSource::PointSet(source.to_owned()))
}

/*fn parse_plot_source(source: &str) -> Result< > {

}*/

impl ReplState {
	pub fn cancel(&mut self) {
		self.cancelled = true;
	}
}

fn make_repl_state() -> ReplState {
	return ReplState{
		namespace: HashMap::new(),
		cancelled: false,
	};
}

pub fn run(root_path: &PathBuf) {
	let mut rl = Editor::<()>::new();
	if rl.load_history("repl_history.txt").is_err() { }

	let mut repl_state = make_repl_state();

	loop {
		let readline = rl.readline("> ");
		match readline {
				Ok(line) => {
						rl.add_history_entry(line.as_str());
						repl_state.handle_line(&line);
				},
				Err(ReadlineError::Interrupted) => {
						println!("CTRL-C");
						//break
				},
				Err(ReadlineError::Eof) => {
						println!("CTRL-D");
						break
				},
				Err(err) => {
						println!("Error: {:?}", err);
						break
				}
		}
	}
	rl.save_history("repl_history.txt").unwrap();
}

