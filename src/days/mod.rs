mod day_01;
mod day_02;

use std::time::Instant;

pub trait Solution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

pub fn run(day: u8) {
    let solution: Box<dyn Solution> = match day {
        1 => Box::new(day_01::Day01{}),
        2 => Box::new(day_02::Day02{}),
        _ => unreachable!()
    };

    // Part 1
    let start = Instant::now();
    println!("{}", solution.part_one());
    let duration = start.elapsed();
    println!("Day {} Part 1 execution time: {:?}", day, duration);

    // Part 2
    let start = Instant::now();
    println!("{}", solution.part_two());
    let duration = start.elapsed();
    println!("Day {} Part 2 execution time: {:?}", day, duration);
}