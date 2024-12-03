use std::error::Error;
use std::{fs, process};

const INPUT_FILE: &str = "input.txt";

fn main() {
    match parse_input() {
        Ok(reports) => {
            solution_one(reports);
        }
        Err(e) => {
            eprintln!("unable to parse input, error: {e}");
            process::exit(1);
        }
    }
}

fn solution_one(reports: Vec<Vec<i32>>) {
    let mut safe_count = 0;

    'outer: for report in reports.iter() {
        let mut is_increasing: u8 = 0; // 0 - not set, 1 - increasing, 2 - decreasing

        for i in 1..report.len() {
            let (cur, prev) = (report[i], report[i - 1]);

            // determine if we are increasing or decreasing if this is first iteration
            if is_increasing == 0 {
                is_increasing = if cur > prev { 1 } else { 2 };
            }

            let diff = (cur - prev).abs();

            // if our diff is too big/small or we switched from increasing/decreasing
            if !((diff > 0 && diff <= 3)
                && ((is_increasing == 1 && cur > prev) || is_increasing == 2 && cur < prev))
            {
                continue 'outer;
            }
        }

        safe_count += 1;
    }

    println!("safe reports: {safe_count}");
}

fn parse_input() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let raw_data = fs::read_to_string(INPUT_FILE)?;

    for line in raw_data.lines() {
        reports.push(
            line.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect(),
        );
    }

    Ok(reports)
}
