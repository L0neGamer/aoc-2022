use std::{fs::File, io::BufReader};
use std::io::BufRead;

pub fn read_lines (filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().into_iter().collect::<Result<Vec<_>,_>>().unwrap()
}
