//! tools for persisting collections of apex coordinates on disk.

use std::collections::{HashMap};
use std::convert::TryInto;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use rug::{Integer, Rational};
use rand::prelude::*;


use geometry::Vec2;
use super::Result;

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
  pub points: Vec<Point>,
  _manager: &'a Manager,
}

#[derive(Debug)]
struct SyncState {

}

#[derive(Debug)]
pub struct Manager {
  root_path: PathBuf,
  index: HashMap<String, SyncState>
}

type Coords = Vec2<Rational>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
  coords: Coords,
  radius: Rational
}

/// A callback that returns 
type PointGenerator = Box<dyn Fn(&mut Metadata) -> Vec<Point>>;

impl Manager {
  /// saves an `ApexSet` with the given name.
  /// 
  /// if `_name` is `None`, a unique name is generated.
  pub fn save(&self, _name: Option<&str>, points_fn: PointGenerator)
      -> Result<ApexSet> {
    let mut metadata = Metadata::new();
    let points = points_fn(&mut metadata);
        
    let name = _name.unwrap_or("test_apex_set");
    std::fs::create_dir_all(&self._dir_path(&name))?;
    let count: u32 = points.len().try_into().unwrap();
    let info: Info = Info{
      name: name.to_string(),
      count: count,
      created: Local::now(),
      metadata: metadata
    };

    { // write points to disk
      let point_file = OpenOptions::new().write(true).create_new(true)
        .open(self._point_path(name))?;
      let mut writer = BufWriter::new(point_file);
      serde_json::to_writer(&mut writer, &points)?;
      writer.flush()?;
    }
  
    { // write apex set info to disk
      let info_file = OpenOptions::new().write(true).create_new(true)
        .open(self._info_path(name))?;
      let mut writer = BufWriter::new(info_file);
      serde_json::to_writer_pretty(&mut writer, &info)?;
      writer.flush()?;
    }
    Ok(ApexSet{info, points, _manager: &self})
  }

  /// loads the `ApexSet` with the given name.
  pub fn load(&self, name: &str) -> Result<ApexSet> {
    let contents = fs::read_to_string(self._info_path(name))?;
    // might want this to be from_reader rather than from_str
    let info: Info = serde_json::from_str(&contents)?;
    Ok(ApexSet{info, points: Vec::new(), _manager: &self})
  }

  fn _dir_path(&self, name: &str) -> PathBuf {
    let mut path = self.root_path.to_owned();
    path.push(name);
    path
  }

  fn _info_path(&self, name: &str) -> PathBuf {
    let mut path = self._dir_path(name);
    path.push("info.json");
    path
  }

  fn _point_path(&self, name: &str) -> PathBuf {
    let mut path = self._dir_path(name);
    path.push("points.json");
    path
  }
}

struct RandomGridCoords {
  grid_density: u32,
  rng: ThreadRng,
  //rand: RandState<'a>,
  denominator: Integer
}

impl RandomGridCoords {
  fn new(grid_density: u32) -> RandomGridCoords {
    // denominator has an extra bit of precision so points can be
    // centered within their grid cell.
    RandomGridCoords{
      grid_density,
      rng: rand::thread_rng(),
      denominator: Integer::from(1) << (grid_density + 1)
    }
  }

  fn rand_int(&mut self, bits: u32) -> Integer {
    let remainder = bits % 32;
    let initial: u32 = 
      if remainder > 0 {
        let bitmask = (1 << remainder) - 1;
        self.rng.gen::<u32>() & bitmask }
      else { 0 };
    
    let block_factor: u64 = 1 << 32;

    let mut result = Integer::from(initial);
    let block_count = bits / 32;
    for _ in 0..block_count {
      result = result * block_factor + self.rng.gen::<u32>()
    }
    result
  }
}

impl Iterator for RandomGridCoords {
  type Item = Vec2<Rational>;

  fn next(&mut self) -> Option<Self::Item> {
    let x_num = self.rand_int(self.grid_density) * 2 + 1;
    let y_num = self.rand_int(self.grid_density - 1) * 2 + 1;
    let x = Rational::from((x_num, self.denominator.clone()));
    let y = Rational::from((y_num, self.denominator.clone()));
    Some(Vec2(x, y))
  }
}

/// generates a random point set with size `count`.
/// 
/// `grid_density` specifies the density of the grid in base-2 log: 
/// there are 2^grid_density grid cells per axis-aligned unit interval.
/// grid cells are chosen uniformly at random among those overlapping the
/// unit-diameter semicircle of obtuse apexes above the base edge
/// (0,0) - (1,0).
pub fn random_from_grid(grid_density: u32, count: u32) -> PointGenerator {
  if grid_density < 1 {
    panic!("random_from_grid requires grid_density > 0");
  }
  //Integer::random_bits(grid_density, &mut rng)
  Box::new(move |metadata| -> Vec<Point> {
    metadata.insert(
      "grid_density".to_string(),
      format!("{}", grid_density).to_string());
      
    // the width / height of a grid cell
    let cell_size = Integer::from(1) << (grid_density + 1);

    // 17/24 > sqrt(2)/2 is the radius bound we need to completely cover a
    // grid cell from its center.
    let radius = Rational::from((17, cell_size * 24));

    let points_iter = RandomGridCoords::new(grid_density)
      .map(|coords| {
        Point{coords, radius: radius.clone()}
      });

    points_iter.take(count as usize).collect()
  })
}

pub fn manager(root_path: PathBuf) -> Manager {
  println!("creating apex_set::Manager with root_path '{}'", root_path.to_str().unwrap_or(""));
  Manager{
    root_path,
    index: HashMap::new()
  }
}