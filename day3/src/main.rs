use regex::Regex;
use std::error::Error;
use std::{fs, process};

const INPUT_FILE: &str = "input.txt";

fn main() {
    match parse_input() {
        Ok(data) => {
            match solution_one(&data) {
                Ok(total) => println!("solution one: {total}"),
                Err(e) => log_err_and_exit("error running solution_one algorithm", e),
            }
            match solution_two(&data) {
                Ok(total) => println!("solution two: {total}"),
                Err(e) => log_err_and_exit("error running solution_two algorithm", e),
            }
        }
        Err(e) => log_err_and_exit("unable to parse input file", e),
    }
}

fn solution_one(data: &str) -> Result<i32, Box<dyn Error>> {
    let re = Regex::new(r"(?s)mul\((\d+),(\d+)\)")?;
    let mut total = 0;

    for (_, [n1, n2]) in re.captures_iter(data).map(|val| val.extract()) {
        total += n1.parse::<i32>()? * n2.parse::<i32>()?;
    }

    Ok(total)
}

fn solution_two(data: &str) -> Result<i32, Box<dyn Error>> {
    let re = Regex::new(r"(?s)(^|do\(\)).*?(don't\(\)|$)")?;
    let mut total = 0;

    for (x, [_do, _dont]) in re.captures_iter(data).map(|val| val.extract()) {
        total += solution_one(x)?
    }

    Ok(total)
}

fn parse_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(INPUT_FILE)?;
    Ok(data)
}

fn log_err_and_exit(message: &str, err: Box<dyn Error>) {
    println!("{message}: {err}");
    process::exit(1);
}
