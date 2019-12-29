use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;

use clap::{Arg, ArgMatches, App, SubCommand};

use data::point_set;

pub struct GnuplotSpec<'a> {
  pub name: &'a str,
  pub points_path: &'a PathBuf,
  pub png_path: &'a PathBuf,
  pub line_weight: f32,
  pub point_size: f32,
}

impl GnuplotSpec<'_> {
  pub fn render(&self) -> String {
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

pub fn run(data_path: &PathBuf) {
  /*let plot_path = data_path.join("plot");
  let pointset_path = plot_path.join(format!("{}.csv", ))
    plot_path.with()
  }
  let spec = GnuplotSpec{
    name: "pointset_name",
    points_path: 
  }
  println!("{}", gnuplot_input());*/
}

pub fn save_points_as_csv(points: &Vec<point_set::Point>, file_path: &PathBuf) -> Result<(), std::io::Error> {
  let file = File::create(file_path)?;
  let mut writer = BufWriter::new(file);
  for p in points {
    write!(writer, "{},{}\n", p.0.to_f64(), p.1.to_f64())?;
  }
  writer.flush()?;
  Ok(())
}