use crate::solution::Solution;
use crate::utils::{ceil, floor};

const RHS: i32 = 100;

pub struct Day {
    rotations: Vec<i32>,
}

impl Solution for Day {
    fn solve_part_1(&self) -> String {
        let mut dial = 50_i32;
        let mut count_zero = 0_i32;
        for &rotation in &self.rotations {
            dial = (dial + rotation).rem_euclid(RHS);
            if dial == 0 {
                count_zero += 1;
            }
        }
        count_zero.to_string()
    }

    fn solve_part_2(&self) -> String {
        let mut dial = 50_i32;
        let mut count_zero = 0_i32;
        for &rotation in &self.rotations {
            let prev_dial = dial;
            dial += rotation;
            if rotation < 0 {
                count_zero += ceil(prev_dial, RHS) - ceil(dial, RHS);
            } else {
                count_zero += floor(dial, RHS) - floor(prev_dial, RHS);
            }
        }
        count_zero.to_string()
    }
}

impl Day {
    pub fn create(input: &str) -> Box<dyn Solution> {
        let rotations: Vec<i32> = input
            .lines()
            .map(|line| {
                let (direction, number) = line.split_at(1);
                let number: i32 = number.parse::<i32>().unwrap();
                match direction {
                    "L" => -number,
                    "R" => number,
                    _ => unreachable!(),
                }
            })
            .collect();
        Box::new(Day { rotations })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "\
            L68\n\
            L30\n\
            R48\n\
            L5\n\
            R60\n\
            L55\n\
            L1\n\
            L99\n\
            R14\n\
            L82";
        let day = Day::create(input);

        assert_eq!(day.solve_part_1(), "3");
    }

    #[test]
    fn test_part_2_example() {
        let input = "\
            L68\n\
            L30\n\
            R48\n\
            L5\n\
            R60\n\
            L55\n\
            L1\n\
            L99\n\
            R14\n\
            L82";
        let day = Day::create(input);

        assert_eq!(day.solve_part_2(), "6");
    }
}
