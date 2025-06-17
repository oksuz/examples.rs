#[cfg(test)]
mod tests {
  use crate::*;
  const CONTENT: &'static str = "\
Rust:
safe, fast, productive
pick three;";
  
  #[test]
  fn test_search_case_insensitive() {
    let query = "duct";
    assert_eq!(vec!["safe, fast, productive"], search_case_insensitive(query, CONTENT))
  }

  #[test]
  fn test_search_case_sensitive() {
    let query = "Rust";
    assert_eq!(vec!["Rust:"], search_case_sensitive(query, CONTENT))
  }

}