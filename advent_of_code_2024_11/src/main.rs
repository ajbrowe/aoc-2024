use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
//use regex::Regex;

fn blink (stones: Vec<usize>) -> Vec<usize> {
    let mut new_stones: Vec<usize> = Vec::new();

    for s in stones {
        if s == 0 {
            new_stones.push(1);
            continue;
        }
        let num_string = s.to_string();
        if num_string.len() % 2 == 0 {
            let middle = num_string.len() / 2;
            new_stones.push(num_string[..middle].parse::<usize>().unwrap_or(0));
            new_stones.push(num_string[middle..].parse::<usize>().unwrap_or(0));
        }
        else {
            new_stones.push(s * 2024)
        }
    }
    new_stones
}

fn blink_hash (stones: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stones = HashMap::with_capacity(stones.len());
    for (&stone, &count) in stones {
        match stone {
            // X 0 stones transform to X 1 stones
            0 => *new_stones.entry(1).or_insert(0) += count,
            _ => {
                let num_string = stone.to_string();
                if num_string.len() % 2 == 0 {
                    let middle = num_string.len() / 2;
                    let a = num_string[..middle].parse::<usize>().unwrap_or(0);
                    let b = num_string[middle..].parse::<usize>().unwrap_or(0);
                    *new_stones.entry(a).or_insert(0) += count;
                    *new_stones.entry(b).or_insert(0) += count;
                }
                else {
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }
    }
    new_stones
}

fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample2");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut stones: Vec<usize> = Vec::new();

    // Process each line
    for line in reader.lines() {
        let line = line?;
        stones = line.split_whitespace().filter_map(|s| s.parse::<usize>().ok()).collect();
        break;
    }

    // Create hash of each stone for p2
    let mut stones_map: HashMap<usize, usize> = HashMap::with_capacity(stones.len());
    for s in &stones {
        *stones_map.entry(*s).or_insert(0) = 1;
    }

    for _ in 0..25 {
        stones = blink(stones);
    }
    let part1_answer: usize = stones.len();

    for _ in 0..75 {
        stones_map = blink_hash(&stones_map);
    }
    println!("There were {} different stones after 75 blinks", stones_map.len());
    let part2_answer: usize = stones_map.values().sum();




    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

