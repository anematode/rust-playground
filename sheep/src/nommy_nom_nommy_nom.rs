use std::fs;

// Relative to sheep/ (current directory)
const DEFAULT_PATH: &'static str = "./stuff/yesyes.txt";

pub fn yes(maybe_path: Option<&String>) {
    let default_path = String::from(DEFAULT_PATH);
    let path = maybe_path.unwrap_or(&default_path);
    let input = fs::read_to_string(path).unwrap();
    println!("{}", input);
}
