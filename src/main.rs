#![allow(dead_code, unused_variables, unused_imports)]
extern crate chrono;
extern crate clap;
//#[macro_use] extern crate maplit;
extern crate rug;
extern crate rand;
extern crate serde;
extern crate serde_json;

mod command_line;
mod data;
mod geometry; 

use std::env;

fn main() {
  /*let args: Vec<String> = env::args().collect();
  //println!("{:?}", args);
  let result = command_line::run(&args[1..]);
  if let Err(err) = result {
    println!("Error: {}", err)
  }*/
  command_line::run();
}