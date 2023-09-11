use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

#[allow(dead_code)]
const EXAMPLE: &str = include_str!("../inputs/example.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("../inputs/input.txt");

pub fn solve() {
    // let input: Vec<i32> = EXAMPLE.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let input: Vec<i32> = INPUT.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    let mut shifter = Shifter::new(&input);
    // println!("{:?}", shifter);
    // println!("{:?}", shifter.get_shifted());

    for (i, n) in input.into_iter().enumerate() {
        shifter.shift(i, n);
    }

    // println!("{:?}", shifter.get_shifted());

    let zero_idx = shifter.get_zero_index();
    println!("0 index {zero_idx}");
    let decrypt = shifter.get_shifted();
    let x = decrypt[(zero_idx + 1000) % decrypt.len()];
    let y = decrypt[(zero_idx + 2000) % decrypt.len()];
    let z = decrypt[(zero_idx + 3000) % decrypt.len()];

    println!(
        "Day 20 part 1: 1000th: {x} | 2000th: {y} | 3000th: {z} | {}",
        x + y + z
    );
}

#[derive(Debug)]
struct Shifter(Vec<(usize, i32)>);

impl Shifter {
    fn new(v: &Vec<i32>) -> Self {
        Self(v.iter().map(i32::clone).enumerate().collect())
    }

    fn shift(&mut self, idx: usize, n: i32) {
        if n == 0 {
            return;
        }
        // println!("find: {:?}", (idx, n));
        // find element el that el.0 == idx and return its shift list index
        let shift_index = self
            .0
            .iter()
            .enumerate()
            .find(|(_, el)| el.0 == idx)
            .map(|(s_i, _)| s_i)
            .unwrap()
            .clone();

        let new_index = self.wrapping_idx_add(shift_index, n);

        if new_index == self.0.len() {
            self.0.push((idx, n));
        } else {
            self.0.insert(new_index, (idx, n));
        }

        if new_index <= shift_index {
            self.0.remove(shift_index + 1);
        } else {
            self.0.remove(shift_index);
        }
        // println!("{:?}", self.get_shifted());
    }

    fn wrapping_idx_add(&self, idx: usize, n: i32) -> usize {
        let len = self.0.len() as isize;

        let new_idx = (idx as isize).add(n as isize);

        if new_idx.is_negative() {
            // get the index of the -nth element
            // e.g v[-1] = v[len - (1%len)]
            return (len - (new_idx.abs() % len)) as usize;
        } else if n.is_negative() && new_idx == 0 {
            return len as usize;
        } else if n.is_negative() {
            return ((new_idx) % len) as usize;
        }

        return ((new_idx + 1) % len) as usize;
    }

    fn get_shifted(&self) -> Vec<i32> {
        self.0.iter().map(|(_, n)| *n).collect()
    }

    fn get_zero_index(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .find(|(_, el)| el.1 == 0)
            .map(|(i, _)| i)
            .unwrap()
    }
}
