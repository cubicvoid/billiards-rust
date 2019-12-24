mod apex_set;

pub fn run(args: &[String]) {
  println!("hello {:?}", args);
  let command = args.first().expect("Expected command");
  match &command[..] {
    "apexSet" => {
      println!("apexSet");
      apex_set::run(&args[1..])
    },
    _ => println!("Unknown command '{}'", command)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run() {
    run(&[]);
  }
}