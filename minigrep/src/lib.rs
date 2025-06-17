use std::{env, fs};

#[allow(dead_code)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough parameters. usage: minigrep query file_path");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
      search_case_insensitive(&config.query, &content)
    } else {
      search_case_sensitive(&config.query, &content)
    };

    for line in result {
      println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&str> = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
} 

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&str> = Vec::new();
  let query = query.to_lowercase();
  for line in content.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
} 

mod tests;