use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample2");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Create a regular expression to match each multiplier
    let p1re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let p2re = Regex::new(r"(?:don't\(\)|do\(\)|mul\(\d{1,3},\d{1,3}\))").unwrap();

    let mut part1_answer: u64 = 0;
    let mut part2_answer: u64 = 0;

    let mut enabled: bool = true;
    // Process each line
    for line in reader.lines() {
        let line = line?;
        for captures in p1re.captures_iter(&line) {
            let x: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
            let y: u64 = captures.get(2).unwrap().as_str().parse().unwrap();
            part1_answer += x * y;
        }

        for captures in p2re.find_iter(&line) {
            let command = captures.as_str();
            if command == "don't()" {
                enabled = false;
            } else if command == "do()" {
                enabled = true;
            } else if command.starts_with("mul(") {
                if enabled {
                    // reuse p1 regex - could have turned this into a reusable function
                    // but... it's AoC not prod code
                    if let Some(caps) = p1re.captures(command) {
                        let x: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
                        let y: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
                        part2_answer += x * y;
                    }
                }

            }
            

        }


    }

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

