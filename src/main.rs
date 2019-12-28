#![allow(dead_code, unused_variables, unused_imports)]
extern crate chrono;
extern crate clap;
extern crate colored;
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
  command_line::run();
}