use std::{fs, io::BufReader, io::BufRead};

pub fn solve() {
    let mut calories = vec![];

    let file = fs::File::open("inputs/day_one.txt").expect("file should exist");
    let buffer = BufReader::new(file);

    let mut sum = 0;

    for line in buffer.lines() {
        match line.expect("should be a line").parse::<i32>() {
            Ok(cal) => {
                sum += cal;                        
            },
            Err(_) => {
                calories.push(sum);
                sum = 0;
            }
        }
    }

    calories.sort();
    calories.reverse();

    println!("DAY 1 PART 1 ANSWER: {}", calories.first().unwrap());

    let top_three: i32 = calories[0..3].iter().sum();

    println!("DAY 1 PART 2 ANSWER: {}", top_three);
}