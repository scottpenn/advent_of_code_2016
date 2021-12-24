use crate::days::Solution;

pub struct Day02 {}

impl Solution for Day02 {

    fn part_one(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_02.txt").unwrap();

        let keypad: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

        let mut code = String::new();

        for line in input.lines() {
            let mut x = 1;
            let mut y = 1;
            for direction in line.chars() {
                match direction {
                    'L' => y = if y == 0 {0} else {y - 1},
                    'R' => y = if y == 2 {2} else {y + 1},
                    'U' => x = if x == 0 {0} else {x - 1},
                    'D' => x = if x == 2 {2} else {x + 1},
                    _ => unreachable!()
                };
            }
            code.push(keypad[x][y]);
        }
        code
    }

    fn part_two(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_02.txt").unwrap();

        let keypad: [[char; 5]; 5] = [['X', 'X', '1', 'X', 'X'],
                                      ['X', '2', '3', '4', 'X'],
                                      ['5', '6', '7', '8', '9'],
                                      ['X', 'A', 'B', 'C', 'X'],
                                      ['X', 'X', 'D', 'X', 'X']];

        let mut code = String::new();

        for line in input.lines() {
            let mut x = 2;
            let mut y = 2;
            for direction in line.chars() {
                match direction {
                    'L' => y = if y == 0 || keypad[x][y - 1] == 'X' {y} else {y - 1},
                    'R' => y = if y == 4 || keypad[x][y + 1] == 'X' {y} else {y + 1},
                    'U' => x = if x == 0 || keypad[x - 1][y] == 'X' {x} else {x - 1},
                    'D' => x = if x == 4 || keypad[x + 1][y] == 'X' {x} else {x + 1},
                    _ => unreachable!()
                };
            }
            code.push(keypad[x][y]);
        }
        code
    }
}