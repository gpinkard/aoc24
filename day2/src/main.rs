use std::error::Error;
use std::{fs, process};

const INPUT_FILE: &str = "input.txt";

#[derive(Debug, PartialEq, Eq)]
enum ReportLevelDirection {
    Increasing,
    Decreasing,
    Unknown,
}

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
    let mut level_direction = ReportLevelDirection::Unknown;

    for i in 1..report.len() {
        let (cur, prev) = (report[i], report[i - 1]);

        // true if the diff between the two points is too large or 0
        let bad_delta = match (cur - prev).abs() {
            0 => true,
            1..=3 => false,
            _ => true,
        };

        // determine whether levels are going up or down if this is first iteration
        level_direction = match level_direction {
            ReportLevelDirection::Unknown => {
                if cur > prev {
                    ReportLevelDirection::Increasing
                } else {
                    ReportLevelDirection::Decreasing
                }
            }
            _direction => _direction,
        };

        // true if we switched directions (ex: went from increasing to decreasing levels)
        let switched_directions = (level_direction == ReportLevelDirection::Increasing
            && cur < prev)
            || (level_direction == ReportLevelDirection::Decreasing && cur > prev);

        // if our diff is too big/small or we switched from increasing/decreasing
        if bad_delta || switched_directions {
            return false;
        }
    }

    true
}

fn solution_one(reports: &[Vec<i32>]) {
    let mut total = 0;
    for report in reports.iter() {
        total = if process_single_report(report) {
            total + 1
        } else {
            total
        };
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
            if process_single_report(&tmp) {
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
