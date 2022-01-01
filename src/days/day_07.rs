use std::collections::HashSet;

use crate::days::Solution;
use arraydeque::{ArrayDeque, Wrapping};

pub struct Day07 {}

impl Solution for Day07 {

    fn part_one(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_07.txt").unwrap();

        let mut num_valid = 0;

        for line in input.lines() {
            let mut abba: ArrayDeque<[_; 4], Wrapping> = ArrayDeque::new();

            let mut hypernet = false;
            let mut valid = false;

            for char in line.chars() {
                match char {
                    '[' => {hypernet = true; abba.clear();},
                    ']' => {hypernet = false; abba.clear();},
                    _ => {
                        abba.push_back(char);
                        if abba.len() == 4 &&
                            abba.get(0) != abba.get(1) &&
                            abba.get(0) == abba.get(3) &&
                            abba.get(1) == abba.get(2) {
                            if hypernet {valid = false; break;} else {valid = true;}
                        }
                    }
                }
            }
            if valid {num_valid += 1;}
        }

        num_valid.to_string()
    }

    fn part_two(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_07.txt").unwrap();

        let mut num_valid = 0;

        for line in input.lines() {
            let mut aba: HashSet<(char, char)> = HashSet::new();
            let mut bab: HashSet<(char, char)> = HashSet::new();
            
            let mut window: ArrayDeque<[_; 3], Wrapping> = ArrayDeque::new();

            let mut hypernet = false;

            for char in line.chars() {
                match char {
                    '[' => {hypernet = true; window.clear();},
                    ']' => {hypernet = false; window.clear();},
                    _ => {
                        window.push_back(char);
                        if window.len() == 3 &&
                            window.get(0) != window.get(1) &&
                            window.get(0) == window.get(2) {
                            if hypernet {
                                bab.insert((*window.get(1).unwrap(), *window.get(0).unwrap()));
                            } else {
                                aba.insert((*window.get(0).unwrap(), *window.get(1).unwrap()));
                            }
                        }
                    }
                }
            }
            if aba.intersection(&bab).count() > 0 {num_valid += 1;}
        }

        num_valid.to_string()
    }
}