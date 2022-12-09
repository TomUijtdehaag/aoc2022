use std::fs;

pub fn read_input(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Unable to read file.");
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();

    lines
}
