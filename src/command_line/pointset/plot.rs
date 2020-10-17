use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::{Arg, ArgMatches, App, SubCommand};
use rug::Rational;

use crate::algebra::{Zero, One};
use crate::billiards::Params;
use crate::billiards::base_edge::BaseEdge;
use crate::data::point_set;
use crate::billiards::singularity::{BaseOrientation, BaseValues};
use crate::data::point_set::{Point, PointSet};
use crate::vector::V2;

type PointIter = Box<dyn Iterator<Item = Point>>;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("plot")
    .about("Plots a specified point set with gnuplot")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the point set to plot")
    )
    .arg(Arg::with_name("filter")
      .long("filter")
      .takes_value(false)
    )
    
}

fn test_path_on_point(p: &Point) -> bool {
  let apex: V2<Rational> = V2::from(p.to_owned());
  let mut params = Params::new(apex);

  //let turns: Vec<i32> = vec![-9, 6, 9, -6];
  //let turns: Vec<i32> = vec![-6, 3, 6, -3];
  let turns: Vec<i32> = vec![-2, 2, 2, -2];
  let mut edge = BaseEdge::new(
    &mut params,
    BaseValues(
      V2(Rational::zero(), Rational::zero()),
      V2(Rational::one(), Rational::zero()),
    ),
    BaseOrientation::Forward,
  );
  let mut left_points = vec![edge.left_apex()];
  let mut right_points = vec![edge.right_apex()];
  for turn in turns {
    if (turn.abs() as u32) > edge.params.max_turn_around(edge.to()) {
      return false;
    }
    edge.step(turn);
    left_points.push(edge.left_apex());
    right_points.push(edge.right_apex());
    if turn > 0 {
      left_points.push(edge.from_coords());
    } else {
      right_points.push(edge.from_coords());
    }
  }
  let offset = left_points.last().unwrap().clone() - left_points.first().unwrap();
  let normal = V2(-offset.1, offset.0);
  let dot_normal = |v: &V2<Rational>| v.dot(&normal);
  let left_offsets = left_points.iter().map(dot_normal);
  let right_offsets = right_points.iter().map(dot_normal);
  left_offsets.min().unwrap() > right_offsets.max().unwrap()

  //p.0.to_owned() * &p.0 - &p.1 < 0
  /*let left = apex.clone() * apex.clone() / left_norm;
  let right = right_edge.clone() * right_edge.clone() / right_norm;
  left.0 > 0 && right.0 > 0*/
  //true
}

pub fn run(data_path: &PathBuf, manager: &point_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  //plot::run();
  match manager.load(name) {
    Err(e) => { eprintln!("couldn't load point set '{}': {}", name, e); },
    Ok(point_set) => {
      let mut points_iter = point_set.points.iter();
      let result = if matches.is_present("filter") {
        _do_plot(data_path, name,
          &mut points_iter.filter(|p| {
            test_path_on_point(&p)
          }))
      } else {
        _do_plot(data_path, name, &mut points_iter)
      };
      if let Err(e) = result {
        eprintln!("error plotting point set '{}': {}", name, e);
      }
    }
  }
}

fn _do_plot<'a, I: Iterator<Item=&'a Point>>(
  data_path: &PathBuf, name: &str, points_iter: &mut I,
) -> Result<(), String> {
  let plot_dir_path = data_path.join("plots");
  std::fs::create_dir_all(&plot_dir_path).map_err(|e| format!("{}", e).to_string())?;

  let points_path = plot_dir_path.join(format!("{}.csv", name));
  save_points_as_csv(points_iter, &points_path)
    .map_err(|e| format!("{}", e).to_string())?;

  let png_path = points_path.with_extension("png");
  let spec = GnuplotSpec{
    name,
    points_path: &points_path,
    png_path: &png_path,
    line_weight: 2.0,
    point_size: 1.4,
  };
  let gnuplot_input = spec.render();
  let mut child = Command::new("gnuplot")
    .stdin(Stdio::piped())
    .spawn()
    .map_err(|e| format!("couldn't start gnuplot: {}", e))?;
  {
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(gnuplot_input.as_bytes())
      .map_err(|e| format!("couldn't connect to gnuplot: {}", e))?;
  }
  let ecode = child.wait()
    .map_err(|e| format!("couldn't get gnuplot results: {}", e))?;
  if !ecode.success() {
    return Err(format!("gnuplot returned error {}", ecode).to_string())
  }
  // copy the generated plot to data/plot.png, where we always keep the
  // most recent plot.
  let copy_to_path = data_path.join("plot.png");
  if std::fs::copy(png_path, copy_to_path).is_err() {
    eprintln!("warning: completed plot couldn't be copied to data/plot.png,
               check data/plots/"); 
  }

  Ok(())
}


struct GnuplotSpec<'a> {
  name: &'a str,
  points_path: &'a PathBuf,
  png_path: &'a PathBuf,
  line_weight: f32,
  point_size: f32,
}

impl GnuplotSpec<'_> {
  fn render(&self) -> String {
    format!(r#"
set terminal png large size 1600,1000 font "Verdana,16"
set datafile separator ","
set output "{}"
set grid
set timestamp ""
set key default
set key box
set key ins vert
set key left top

set size ratio 0.625
set xrange [0:1.0]
set yrange [0:0.625]

set timestamp

set style line 1 lc rgb '#666699' lw {} pt 6 ps {}

set object 1 circle at 0.5,0 size 0.5 behind
plot \
  "{}" with points ls 1 title "{}"
quit
"#,
      format!("{}", self.png_path.to_str().unwrap()),
      format!("{}", self.line_weight),
      format!("{}", self.point_size),
      format!("{}", self.points_path.to_str().unwrap()),
      self.name
    ).to_string()
  }
  }

pub fn save_points_as_csv<'a, I: Iterator<Item=&'a Point>>(
  points_iter: &mut I, file_path: &PathBuf,
) -> Result<(), std::io::Error> {
  let mut count = 0;
  let file = File::create(file_path)?;
  let mut writer = BufWriter::new(file);
  for p in points_iter {
    write!(writer, "{},{}\n", p.0.to_f64(), p.1.to_f64())?;
    count += 1;
  }
  writer.flush()?;
  eprintln!("saved {} points", count);
  Ok(())
}