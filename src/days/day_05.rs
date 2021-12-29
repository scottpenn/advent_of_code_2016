use crate::days::Solution;
use md5;

pub struct Day05 {}

impl Solution for Day05 {

    fn part_one(&self) -> String {
        let input = "uqwqemis";

        (1..).map(|n: u32| format!("{:x}", md5::compute(format!("{}{}", input, n))))
            .filter(|m| &m[..5] == "00000")
            .map(|m| m.chars().nth(5).unwrap())
            .take(8)
            .collect()
    }

    fn part_two(&self) -> String {
        let input = "uqwqemis";

        let mut code = ['x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'];

        let mut n: i32 = 0;

        loop {
            n += 1;
            let hash = format!("{:x}", md5::compute(format!("{}{}", input, n)));
            if &hash[..5] != "00000" {continue;}
            if let Some(i) = hash.chars().nth(5).unwrap().to_digit(10) {
                if i < 8 && code[i as usize] == 'x' {
                    code[i as usize] = hash.chars().nth(6).unwrap();
                }
            }

            if code.into_iter().all(|c| c != 'x') {break;}
        }

        code.iter().collect()
    }
}