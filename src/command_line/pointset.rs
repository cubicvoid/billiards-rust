mod tabulator;

use std::path::PathBuf;

use clap::{Arg, ArgMatches, App, SubCommand};

use data::point_set;

use self::tabulator::Tabulator;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("pointset")
    .about("Manipulates sets of random points")
    .subcommands(vec![
      subcommand_create(),
      subcommand_list(),
      subcommand_print(),
      subcommand_delete(),
    ])
}

pub fn subcommand_create<'a, 'b>() -> App<'a, 'b> {
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

pub fn subcommand_list<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("list")
    .about("Lists all point sets")
}

pub fn subcommand_print<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("print")
    .about("Prints a specified point set")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the point set to print.")
    )
}

pub fn subcommand_delete<'a, 'b>() -> App<'a, 'b> {
  SubCommand::with_name("delete")
    .about("Deletes a point set")
    .arg(Arg::with_name("name")
      .index(1)
      .required(true)
      .help("The name of the point set to delete.")
    )
}

pub fn run(root_path: &PathBuf, matches: &ArgMatches) {
  let point_set_manager = {
    let mut path = root_path.to_owned();
    //let mut path: PathBuf = env::current_dir().unwrap();//?;
    path.push("data");
    path.push("point_set");
    point_set::manager(path)
  };
  match matches.subcommand() {
    ("create", Some(sub_m)) => { run_create(&point_set_manager, sub_m) },
    ("list", Some(sub_m)) => { run_list(&point_set_manager, sub_m) },
    ("print", Some(sub_m)) => { run_print(&point_set_manager, sub_m) },
    ("delete", Some(sub_m)) => { run_delete(&point_set_manager, sub_m) },
    _ => { eprintln!("{}", matches.usage()); }
  }
}

pub fn run_create(manager: &point_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  let count = matches.value_of("count").unwrap().parse::<u32>().unwrap();
  let grid_density = matches.value_of("grid_density").map(|s| s.parse::<u32>().unwrap()).unwrap_or(32);
  let overwrite = matches.is_present("overwrite");
  let point_generator = point_set::random_from_grid(grid_density, count);
  if let Err(e) = manager.save(name, overwrite, point_generator) {
    eprintln!("couldn't create point set: {}", e);
  } else {
    eprintln!("saved {} points as '{}'", count, name);
  }
}

pub fn run_list(manager: &point_set::Manager, matches: &ArgMatches) {
  let mut point_sets = manager.list().unwrap();
  let mut table = Tabulator::new(vec![
    String::from("name"),
    String::from("count"),
    String::from("created")]);
  point_sets.sort_by(|a,b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
  for info in point_sets {
    let count = format!("{}", info.count);
    let created = info.created.to_rfc2822().to_string();
    table.append(vec![info.name.to_string(), count, created]);
  }
  table.display();
}

pub fn run_print(manager: &point_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  let result = manager.load(name);
  match result {
    Err(e) => { eprintln!("couldn't load point set '{}': {}", name, e); },
    Ok(point_set) => {
      for p in point_set.points {
        println!("{},{}", p.0.to_f64(), p.1.to_f64());
      }
    }
  }
}

pub fn run_delete(manager: &point_set::Manager, matches: &ArgMatches) {
  let name = matches.value_of("name").unwrap();
  eprintln!("deleting point sets hasn't been implemented");
  //eprintln!("deleting point set '{}'", name);
}