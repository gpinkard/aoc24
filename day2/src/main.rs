use std::error::Error;
use std::{fs, process};

const INPUT_FILE: &str = "input.txt";

fn main() {
    match parse_input() {
        Ok(reports) => {
            solution_one(&reports);
            solution_two(&reports);
        }
        Err(e) => {
            eprintln!("unable to parse input, error: {e}");
            process::exit(1);
        }
    }
}

fn process_single_report(report: &[i32]) -> bool {
    let mut is_increasing: u8 = 0; // 0 - not set, 1 - increasing, 2 - decreasing

    for i in 1..report.len() {
        let (cur, prev) = (report[i], report[i - 1]);

        // determine if we are increasing or decreasing if this is first iteration
        if is_increasing == 0 {
            is_increasing = if cur > prev { 1 } else { 2 };
        }

        let diff = (cur - prev).abs();
        let switched_directions =
            (is_increasing == 1 && cur > prev) || (is_increasing == 2 && cur < prev);

        // if our diff is too big/small or we switched from increasing/decreasing
        if !((diff > 0 && diff <= 3) && switched_directions) {
            return false;
        }
    }

    true
}

fn solution_one(reports: &[Vec<i32>]) {
    let mut total = 0;
    for report in reports.iter() {
        let is_safe = process_single_report(report);
        total = if is_safe { total + 1 } else { total };
    }

    println!("solution one: {total}");
}

fn solution_two(reports: &[Vec<i32>]) {
    let mut total = 0;

    for report in reports.iter() {
        for i in 0..report.len() {
            // brute force by removing each element from the report
            // until we find one that works or exhaust every element
            let mut tmp = report.clone();
            tmp.remove(i);
            let is_safe = process_single_report(&tmp);
            if is_safe {
                total += 1;
                break;
            }
        }
    }

    println!("solution two: {total}");
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
