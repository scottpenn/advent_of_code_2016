mod day_01;

use std::time::Instant;

pub fn run(day: u8) {
    let solution = match day {
        1 => day_01::part_1,
        _ => day_01::part_1
    };

    let start = Instant::now();
    println!("{}", solution());
    let duration = start.elapsed();
    println!("Day {} execution time: {:?}", day, duration);
}