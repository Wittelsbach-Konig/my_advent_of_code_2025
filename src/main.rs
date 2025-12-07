use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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
    let result = read_input(Path::new("input/input.txt"))?;

    println!("{result}");

    Ok(())
}
