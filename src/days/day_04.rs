use itertools::Itertools;
use regex::Regex;
use counter::Counter;

use crate::days::Solution;

pub struct Day04 {}

impl Solution for Day04 {

    fn part_one(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_04.txt").unwrap();
        
        let re = Regex::new(r"(\d*)\[([a-z]*)\]").unwrap();

        let mut id_sums = 0;

        for line in input.lines() {
            let room: Vec<_> = line.split("-").collect();

            let mut counter: Counter<char, usize> = Counter::new();

            for name in &room[..room.len() - 1] {
                counter.extend(name.chars());
            }

            let code = &counter.most_common_ordered()
                                        .iter()
                                        .take(5)
                                        .map(|c| c.0)
                                        .collect::<String>();

            let id_and_checksum = re.captures(&room[room.len() - 1]).unwrap();

            if &id_and_checksum[2] == code {
                id_sums += &id_and_checksum[1].parse().unwrap();
            }
        }

        id_sums.to_string()
    }

    fn part_two(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_04.txt").unwrap();
        
        let re = Regex::new(r"(\d*)\[([a-z]*)\]").unwrap();

        let mut id = 0;

        for line in input.lines() {
            let room: Vec<_> = line.split("-").collect();

            id = re.captures(&room[room.len() - 1]).unwrap()[1].parse().unwrap();
            let a = 'a' as u32;
            
            let real_name = room[..room.len() - 1].iter().map(|name| {
                name.chars()
                    .map(|c| ((((c as u32) - a + id) % 26) + a) as u8 as char)
                    .collect::<String>()
                }
            ).join(" ");

            if real_name == "northpole object storage" {
                break;
            }
        }

        id.to_string()
    }
}