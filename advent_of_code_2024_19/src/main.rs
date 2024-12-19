use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use regex::Regex;
use std::collections::{HashSet, HashMap};

fn find_matching_designs(designs: Vec<String>, patterns: Vec<String>) -> Vec<String> {
    let patterns_set: HashSet<String> = patterns.into_iter().collect();
    let mut matching_designs = Vec::new();

    for design in designs {
        let mut memo: HashMap<String, bool> = HashMap::new();
        if can_construct(&design, &patterns_set, &mut memo) {
            matching_designs.push(design.to_string());
        }
    }

    matching_designs
}

fn can_construct(design: &str, patterns: &HashSet<String>, memo: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&cached) = memo.get(design) {
        return cached;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            if can_construct(remaining, patterns, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(design.to_string(), false);
    false
}

fn find_all_matching_designs(designs: Vec<String>, patterns: Vec<String>) -> usize {
    let patterns_set: HashSet<String> = patterns.into_iter().collect();
    let mut matching_designs: usize = 0;
    
    for design in designs {
        let mut memo: HashMap<String, usize> = HashMap::new();
        matching_designs += can_construct_combinations(&design, &patterns_set, &mut memo);
    }

    matching_designs
}


fn can_construct_combinations (design: &str, patterns: &HashSet<String>, memo: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&cached) = memo.get(design) {
        return cached;
    }

    let mut count: usize = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            count += can_construct_combinations(remaining, patterns, memo); 
        }
    }

    memo.insert(design.to_string(), count);
    count
}



fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Process each line
    let mut patterns_read: bool = false;
    let mut patterns: Vec<String> = Vec::new();
    let mut designs: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            patterns_read = true;
            continue;
        }

        if !patterns_read {
            patterns.extend(line.split(", ").map(String::from));
        }
        else {
            designs.push(line);
        }
    }

    let result = find_matching_designs(designs.clone(), patterns.clone());
    let part1_answer = result.len();
    let part2_answer = find_all_matching_designs(designs.clone(), patterns.clone());

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

