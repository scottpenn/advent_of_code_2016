use crate::days::Solution;
use counter::Counter;

pub struct Day06 {}

impl Solution for Day06 {

    fn part_one(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_06.txt").unwrap();

        let mut counters: Vec<Counter<char, u32>> = vec![Counter::new(); 8];

        input.lines().for_each(|line| {
            line.char_indices().for_each(|(i, c)| {
                counters[i][&c] += 1;
            })
        });

        counters.iter()
            .map(|c| c.most_common().first().unwrap().0)
            .collect()
    }

    fn part_two(&self) -> String {
        let input = std::fs::read_to_string("src/inputs/day_06.txt").unwrap();

        let mut counters: Vec<Counter<char, u32>> = vec![Counter::new(); 8];

        input.lines().for_each(|line| {
            line.char_indices().for_each(|(i, c)| {
                counters[i][&c] += 1;
            })
        });

        counters.iter()
            .map(|c| c.most_common().last().unwrap().0)
            .collect()
    }
}