use std::fs;
use aoc2023::internal::day04::*;

fn main() {
    let input = fs::read_to_string("./data/day4.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}
