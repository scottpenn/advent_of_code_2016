use crate::days::Solution;
use itertools::Itertools;

pub struct Day03 {}

impl Solution for Day03 {

    fn part_one(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_03.txt").unwrap();

        let mut valid = 0;
        for line in input.lines() {
            let sides: Vec<u32> = line.split_whitespace()
                                      .map(|s| s.parse().unwrap())
                                      .sorted()
                                      .collect();

            valid += if sides[0] + sides[1] > sides[2] {1} else {0};
        }

        format!("{}", valid)
    }

    fn part_two(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_03.txt").unwrap();

        let mut valid = 0;
        for lines in &input.lines().chunks(3) {
            let mut triangles: [[u32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
            let mut y = 0;
            for line in lines.into_iter() {
                let mut x = 0;
                for side in line.split_whitespace() {
                    triangles[x][y] = side.parse().unwrap();
                    x += 1;
                }
                y += 1;
            }

            for mut triangle in triangles.into_iter() {
                triangle.sort();
                valid += if triangle[0] + triangle[1] > triangle[2] {1} else {0};
            }
        }

        format!("{}", valid)
    }
}