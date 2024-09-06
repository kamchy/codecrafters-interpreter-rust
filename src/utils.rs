use std::fs;

pub(crate) fn contents(filename: &str) -> String {
  fs::read_to_string(filename).unwrap_or_else(|_| {
      eprintln!("Failed to read file {}", filename);
      String::new()
  })
}