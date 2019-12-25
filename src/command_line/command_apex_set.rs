use super::CommandLineError;
use crate::data::apex_set;

pub fn run(args: &[String]) -> Result<(), CommandLineError> {
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

fn create(args: &[String]) -> Result<(), CommandLineError> {
  let apex_set = apex_set::new_apex_set();
  println!("apexSet create: {:?}", apex_set);
  Ok(())
}

