use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  // cargo run <query> <filename>
  let (query, filename) = parse_config(&args);

  println!("In file: {:#?}", filename);

  let contents = fs::read_to_string(filename)
    .expect("Something went wrong when reading the file");

  println!("With text: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
  let query = &args[1];
  let filename = &args[2];

  (query, filename)
}
