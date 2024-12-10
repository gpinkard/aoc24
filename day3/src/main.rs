use regex::Regex;
use std::error::Error;
use std::{fs, process};

const INPUT_FILE: &str = "input.txt";

fn main() {
    match parse_input() {
        Ok(data) => {
            solution_one(&data);
            // solution_two(&data);
        }
        Err(e) => {
            println!("error parsing input: {e}");
            process::exit(1);
        }
    }
}

fn solution_one(data: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;

    for (_, [n1, n2]) in re.captures_iter(data).map(|elem| elem.extract()) {
        total += n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap();
    }

    println!("total: {total}");
}

/*
fn solution_two(data: &String) {
    let re = Regex::new("").unwrap();
    println!("data: {data}");
}
*/

fn parse_input() -> Result<String, Box<dyn Error>> {
    let data = fs::read_to_string(INPUT_FILE)?;
    Ok(data)
}
