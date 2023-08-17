use std::{fs, io::{BufReader, BufRead}};


pub fn get_file_lines_iter(path: &str) -> std::io::Lines<BufReader<fs::File>> {
    let file = fs::File::open(path).expect("File should exist");

    BufReader::new(file).lines()
}