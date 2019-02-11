use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string")
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name")
    };

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
  }
}

//  For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait,
//  but we don’t have to specify what particular type the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? will return the error value from the current function for the caller to handle
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  // using () like this is the idiomatic way to indicate that
  // we’re calling run for its side effects only; it doesn’t return a value we need.
  Ok(())
}

// This is important! The data referenced by a slice needs to be valid for the reference to be valid; 
// if the compiler assumes we’re making string slices of query rather than contents,
// it will do its safety checking incorrectly
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents.lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    )
  }

  #[test]
  fn case_insensitive() {
    let query = "ruSt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_insensitive(query, contents)
    )
  }
}