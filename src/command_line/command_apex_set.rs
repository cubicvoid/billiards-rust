use std::env;
use std::path::PathBuf;

use data::apex_set;

use super::CommandLineError;
use super::Result;

pub fn run(args: &[String]) -> Result<()> {
  match args.first() {
    None => {
      Err(CommandLineError::new("apexSet: expected command"))
    },
    Some(command) => {
      match &command[..] {
        "create" => {
          println!("create");
          create(&args[1..])
        },
        "delete" => {
          println!("delete");
          Ok(())
        },
        "list" => {
          println!("list");
          Ok(())
        },
        _ => {
          Err(CommandLineError::new(
            &format!("apexSet: Unknown command '{}'", command)))
        }
      }
    },
  }
}

struct CreateArgs<'a> {
  name: Option<&'a str>,
  grid_density: Option<u64>,
  count: Option<u32>,
}

impl<'a> CreateArgs<'a> {
  fn parse(_args: &[String]) -> CreateArgs {
    CreateArgs{
      name: Some("testSet"),
      grid_density: Some(1000000000),
      count: Some(100),
    }
  }
}

fn create(args: &[String]) -> Result<()> {
  let apex_set_manager = {
    let mut path: PathBuf = env::current_dir()?;
    path.push("data");
    path.push("apex_set");
    apex_set::manager(path)
  };
  let create_args = CreateArgs::parse(args);
  let apex_set = apex_set_manager.save(
    create_args.name,
    apex_set::random_from_grid(
      create_args.grid_density.unwrap_or(1000000000),
      create_args.count.unwrap_or(100)));
  println!("apexSet create: {:?}", apex_set);
  Ok(())
}

