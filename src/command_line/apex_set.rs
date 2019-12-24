

pub fn run(args: &[String]) {
  let command = args.first().expect("apexSet: Expected command");
  match &command[..] {
    "create" => {
      println!("create");
    },
    "delete" => {
      println!("delete");
    },
    "list" => {
      println!("list");
    },
    _ => println!("apexSet: Unknown command '{}'", command)
  }
}