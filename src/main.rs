use std::{
    error::Error,
    fs::{File, read_to_string},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use aoc2025::day_1;

fn read_input(path: &Path) -> Result<i32, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file); // * Создаём буфер
    let mut store = 50_i32;
    let mut password = 0_i32;
    for line in reader.lines() {
        let line_ = line?;
        let prefix = &line_[..1];
        let number_part = &line_[1..];
        let number: i32 = number_part.parse::<i32>()?;

        store = match prefix.chars().next().unwrap() {
            'L' => (store - number).rem_euclid(100),
            'R' => (store + number).rem_euclid(100),
            _ => panic!(),
        };

        if store == 0 {
            password += 1;
        }
    }

    Ok(password)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = read_input(Path::new("input/input_1.txt"))?;

    let input_1 = read_to_string(PathBuf::from("input/input_1.txt"))?;

    println!("my first solution: {}", result);

    let day1 = day_1::Day::create(input_1.as_str());

    println!("Day {}, part 1: {}", 1, day1.solve_part_1());
    println!("Day {}, part 2: {}", 1, day1.solve_part_2());

    Ok(())
}
