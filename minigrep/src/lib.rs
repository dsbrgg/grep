use std::fs;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 { return Err("Not enough arguments"); }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

//  For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait,
//  but we don’t have to specify what particular type the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? will return the error value from the current function for the caller to handle
  let contents = fs::read_to_string(config.filename)?;

  println!("With text: \n{}", contents);

  // using () like this is the idiomatic way to indicate that
  // we’re calling run for its side effects only; it doesn’t return a value we need.
  Ok(())
}

// This is important! The data referenced by a slice needs to be valid for the reference to be valid; 
// if the compiler assumes we’re making string slices of query rather than contents,
// it will do its safety checking incorrectly
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    )
  }
}