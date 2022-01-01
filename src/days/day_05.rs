use crate::days::Solution;
use md5;
use hex;

pub struct Day05 {}

impl Solution for Day05 {

    fn part_one(&self) -> String {
        let input = "uqwqemis";

        (1..).map(|n: u32| hex::encode(md5::compute([input, &n.to_string()].join("")).to_ascii_lowercase()))
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
            let hash = hex::encode(md5::compute([input, &n.to_string()].join("")).to_ascii_lowercase());
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