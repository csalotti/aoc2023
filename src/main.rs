mod day1;

use day1::part_one;
use std::{fs::File, io::BufRead, io::BufReader};

use crate::day1::part_two;

fn day_1() {
    let file = File::open("inputs/day1.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(Result::unwrap).collect();
    println!("Part one : {}", part_one(&lines));
    println!("Part two : {}", part_two(&lines));
}

fn main() {
    day_1();
}
