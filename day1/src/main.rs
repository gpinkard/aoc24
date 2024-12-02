use std::collections::HashMap;
use std::error::Error;
use std::{fs, process};

const INPUT_FILE: &str = "input.txt";

fn main() {
    match parse_input() {
        Ok((left_list, right_list)) => {
            solution_one(&left_list, &right_list);
            solution_two(&left_list, &right_list);
        }
        Err(e) => {
            eprintln!("unable to parse input, error: {e}");
            process::exit(1);
        }
    }
}

fn parse_input() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let raw_contents = fs::read_to_string(INPUT_FILE)?;
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    // parse inputs into left and right lists
    for line in raw_contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        left_list.push(parts[0].parse::<i32>()?);
        right_list.push(parts[1].parse::<i32>()?);
    }

    // sort ascending order
    left_list.sort();
    right_list.sort();

    Ok((left_list, right_list))
}

fn solution_one(left_list: &[i32], right_list: &[i32]) {
    let mut total: i32 = 0;

    for i in 0..left_list.len() {
        total += (left_list[i] - right_list[i]).abs();
    }

    println!("total distance: {total}");
}

fn solution_two(left_list: &[i32], right_list: &[i32]) {
    // map elements that appear in both lists to number of occurances in right_list
    let mut similarity_map: HashMap<i32, i32> = HashMap::new();

    for left_elem in left_list.iter() {
        if similarity_map.contains_key(left_elem) {
            // we have already counted the occurances of this element previously
            break;
        }

        let mut occurrences: i32 = 0;
        for right_elem in right_list.iter() {
            if left_elem == right_elem {
                occurrences += 1;
            }
        }
        similarity_map.insert(*left_elem, occurrences);
    }

    let mut total: i32 = 0;
    for elem in left_list.iter() {
        total += elem * similarity_map.get(elem).unwrap();
    }

    println!("similarity score: {total}");
}
