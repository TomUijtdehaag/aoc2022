use std::fs;

fn main() {
    cmd_lines = fs::read_to_string("input.txt".to_string()).expect();
}

struct Folder {
    parent: Folder,
    size: i32,
}
