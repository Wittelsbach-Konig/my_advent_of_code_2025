use std::ops::RangeInclusive;

use crate::solution::Solution;

pub struct Day {
    id_ranges: Vec<RangeInclusive<u64>>,
}

impl Solution for Day {
    fn solve_part_1(&self) -> String {
        self.id_ranges
            .iter()
            .map(|range| {
                range
                    .clone()
                    .map(|id| {
                        let s = id.to_string();
                        if s.len().is_multiple_of(2) && s[..s.len() / 2].repeat(2) == s {
                            id
                        } else {
                            0
                        }
                    })
                    .sum::<u64>()
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_part_2(&self) -> String {
        let prod = 0_u64;

        prod.to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Solution> {
        let id_ranges = input
            .trim()
            .split(',')
            .map(|range| {
                let mut parts = range.trim().split('-');
                let start = parts.next().unwrap().parse::<u64>().unwrap();
                let end = parts.next().unwrap().parse::<u64>().unwrap();
                RangeInclusive::new(start, end)
            })
            .collect::<Vec<RangeInclusive<u64>>>();

        Box::new(Day { id_ranges })
    }
}
