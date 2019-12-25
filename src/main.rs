//extern crate billiards;

use std::env;

mod command_line;
mod data;
mod geometry; 

fn main() {
  let args: Vec<String> = env::args().collect();
  //println!("{:?}", args);
  let result = command_line::run(&args[1..]);
  if let Err(err) = result {
    println!("Error: {}", err)
  }
}