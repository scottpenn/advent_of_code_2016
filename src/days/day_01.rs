use crate::days::Solution;
use std::collections::HashSet;

pub struct Day01();

impl Solution for Day01 {
    fn part_one(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_01.txt").unwrap();
        let instructions : Vec<&str> = input.split(", ").collect();

        let modifiers = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut direction = 0;

        let mut x = 0;
        let mut y = 0;

        for instruction in instructions.into_iter() {
            direction = match &instruction[..1] {
                "L" => if direction == 0 {3} else {direction - 1},
                "R" => if direction == 3 {0} else {direction + 1},
                _ => unreachable!()
            };
            let distance: i32 = instruction[1..].parse().unwrap();
            x += distance * modifiers[direction].0;
            y += distance * modifiers[direction].1;
        }

        let blocks = x.abs() + y.abs();
        blocks.to_string()
    }

    fn part_two(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_01.txt").unwrap();
        let instructions : Vec<&str> = input.split(", ").collect();

        let mut x = 0;
        let mut y = 0;

        let mut locations: HashSet<(i32, i32)> = HashSet::from([(x, y)]);

        let modifiers = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut direction = 0;

        for instruction in instructions.into_iter() {
            direction = match &instruction[..1] {
                "L" => if direction == 0 {3} else {direction - 1},
                "R" => if direction == 3 {0} else {direction + 1},
                _ => unreachable!()
            };
            let distance: i32 = instruction[1..].parse().unwrap();
            for _ in 0..distance {
                x += 1 * modifiers[direction].0;
                y += 1 * modifiers[direction].1;
                if locations.contains(&(x, y)) {
                    let blocks = x.abs() + y.abs();
                    return blocks.to_string()
                } else {
                    locations.insert((x, y));
                }
            }
        }
        "Not Found".to_string()
    }
}