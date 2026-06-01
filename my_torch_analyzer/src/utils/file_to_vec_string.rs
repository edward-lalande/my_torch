use std::fs;

pub fn read_lines_to_vec(filename: String) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect()
}
