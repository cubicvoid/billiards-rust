#![allow(dead_code, unused_variables, unused_imports)]

mod algebra;
mod billiards;
mod command_line;
mod data;
mod geometry;
mod repl;
mod util;
mod vector;

use std::env;
use std::path::{Path, PathBuf};

fn find_root_from_path(mut path: PathBuf) -> Option<PathBuf> {
	let mut filepath = path.clone();
	filepath.push(".billiards_root");
	if filepath.is_file() {
		return Some(path);
	}
	if !path.pop() {
		return None;
	}
	find_root_from_path(path)
}

fn main() {
	let current_path = env::current_dir()
		.expect("couldn't access current directory");
	let root_path = find_root_from_path(current_path).expect(
		"Couldn't find the file '.billiards_root' in this directory or any \
		of its ancestors. Set BILLIARDS_ROOT to a valid path to use this \
		program outside the repository directory."
	);
	eprintln!("working directory: {}", root_path.to_str().unwrap_or("<unknown>"));
	command_line::run(&root_path);
}