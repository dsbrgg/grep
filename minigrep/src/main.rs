use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
  query: String,
  filename: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 { return Err("Not enough arguments"); }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  // cargo run <query> <filename>
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  println!("Searching for: {:#?}", config.query);  
  println!("In file: {:#?}", config.filename);

  if let Err(e) = run(config) {
    println!("Application error: {}", e);

    process::exit(1);
  }
}

//  For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait,
//  but we don’t have to specify what particular type the return value will be
fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? will return the error value from the current function for the caller to handle
  let contents = fs::read_to_string(config.filename)?;

  println!("With text: \n{}", contents);

  // using () like this is the idiomatic way to indicate that
  // we’re calling run for its side effects only; it doesn’t return a value we need.
  Ok(())
}
