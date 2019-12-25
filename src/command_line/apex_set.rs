use super::CommandLineError;

pub fn run(args: &[String]) -> Result<(), CommandLineError> {
  match args.first() {
    None => {
      Err(CommandLineError::new("apexSet: expected command"))
    },
    Some(command) => {
      match &command[..] {
        "create" => {
          println!("create");
          Ok(())
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