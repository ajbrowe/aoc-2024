use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn is_ordered(rules: &HashMap<u64, Vec<u64>>, updates: &[u64]) -> bool {

    for (index, &update) in updates.iter().enumerate() {
        if let Some(rule_values) = rules.get(&update) {
            for &rule_value in rule_values {
                if updates[..index].contains(&rule_value) {
                    return false;
                }
            }
        }
    }
    true
}

fn reordered(rules: &HashMap<u64, Vec<u64>>, updates: &[u64]) -> Vec<u64> {

    let mut reordered_updates: Vec<u64> = Vec::new();
    for &update in updates {
        reordered_updates.push(update);
        if let Some(rule_values) = rules.get(&update) {
            for &rule_value in rule_values {
                if let Some(position) = reordered_updates.iter().position(|&v| v == rule_value) {
                    reordered_updates.remove(position);
                    reordered_updates.push(rule_value);
                }
            }
        }
    }
    if !is_ordered(&rules, &reordered_updates) {
        reordered_updates = reordered(&rules, &reordered_updates);
    }
    reordered_updates
}


fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: u64 = 0;

    let mut part2_answer: u64 = 0;

    let mut rules_complete: bool = false;

    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    // Process each line
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            rules_complete = true;
            continue;
        }

        if !rules_complete {
            if let Some((a, b)) = line.split_once("|") {
                let i: u64 = a.parse().unwrap();
                let j: u64 = b.parse().unwrap();

                rules.entry(i).or_insert_with(Vec::new).push(j);
            }
        } else {
            let updates: Vec<u64> = line.split(",")
                .map(|s| s.parse::<u64>().expect("Invalid integer"))
                .collect();
            if is_ordered(&rules, &updates) {
                let middle_index = updates.len() / 2;
                let middle_value = updates[middle_index];
                part1_answer += middle_value;
            } else {
                let reordered = reordered(&rules, &updates);
                let middle_index = reordered.len() / 2;
                let middle_value = reordered[middle_index];
                part2_answer += middle_value;

            }
        }
    }

   
    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

