use std::fs;

pub(crate) fn contents(file_path: &str) -> String {
   fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
}