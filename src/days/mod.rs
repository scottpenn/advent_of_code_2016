mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

use std::time::Instant;

pub trait Solution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

pub fn run(day: u8) {
    let solution: Box<dyn Solution> = match day {
        1 => Box::new(day_01::Day01{}),
        2 => Box::new(day_02::Day02{}),
        3 => Box::new(day_03::Day03{}),
        4 => Box::new(day_04::Day04{}),
        5 => Box::new(day_05::Day05{}),
        6 => Box::new(day_06::Day06{}),
        7 => Box::new(day_07::Day07{}),
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