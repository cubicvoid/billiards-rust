//! tools for persisting collections of apex coordinates on disk.

use std::collections::{HashMap};
use std::error;
use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

type Metadata = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
  pub name: String,
  pub count: u32,
  pub created: DateTime<Local>,
  pub metadata: Metadata,
}

#[derive(Debug)]
pub struct ApexSet<'a> {
  pub info: Info,
  _manager: &'a Manager,
}

#[derive(Debug)]
struct State {

}

#[derive(Debug)]
pub struct Manager {
  index: HashMap<String, State>
}

pub struct Point {

}

impl Manager {
  /// saves an `ApexSet` with the given name.
  /// 
  /// if `_name` is `None`, a unique name is generated.
  pub fn save<F>(&self, _name: Option<&str>, points_fn: F) -> Option<&ApexSet>
    where F: Fn(&mut Metadata) -> Vec<Point> {
    None
  }

  pub fn load(&self, _name: &str) -> Option<&ApexSet> {
    //let info: Info = serde_json::from_str(data)?;

    None
  }
}

pub fn random_from_grid(
  grid_density: u64, count: u32
) -> Box<dyn Fn(&mut Metadata) -> Vec<Point>> {
  Box::new(move |metadata| -> Vec<Point> {
    metadata.insert(
      "grid_density".to_string(),
      format!("{}", grid_density).to_string());
    (0..count).map(|_| { Point{} }).collect()
  })
}

pub fn manager(root_path: PathBuf) -> Manager {
  println!("creating apex_set::Manager with root_path '{}'", root_path.to_str().unwrap_or(""));
  Manager{
    index: HashMap::new()
  }
}