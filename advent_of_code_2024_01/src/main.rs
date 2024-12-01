use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;


fn main() -> io::Result<()> {

    // Initialise Arrays
    let mut left:  Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Create a regular expression to split each line
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    // Process each line
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let left_value: i32 = caps[1].parse().unwrap();
            let right_value: i32 = caps[2].parse().unwrap();
            left.push(left_value);
            right.push(right_value);
        }
    }

    // Sort the arratys
    left.sort();
    right.sort();

    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut part1_answer: i32 = 0;

    // Create a HashMap for the right list
    let mut right_count: HashMap<i32, i32> = HashMap::new();
    // Find the count for each distinct value
    for &value in &right {
        *right_count.entry(value).or_insert(0) += 1;
    }

    let mut part2_answer: i32 = 0;

    let mut left_val = left_iter.next();
    while left_val.is_some() {
        let right_val = right_iter.next();
        match(left_val, right_val) {
            (Some(&l), Some(&r)) => {
                let difference = (l - r).abs();
                part1_answer += difference;

                let r_count = right_count.get(&l).copied().unwrap_or(0);
                let similarity_score = l * r_count;
                part2_answer += similarity_score;
                left_val = left_iter.next();
            }
            (Some(_), None) => break,
            (None, Some(_)) => break,
            (None, None) => break,
        }

    }

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

