use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use regex::Regex;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    A: Point,
    B: Point,
    prize: Point,
}

// Parsing function to extract values from the lines
fn parse_claw_machine(lines: &[String]) -> Option<Machine> {
    if lines.len() < 3 {
        return None;
    }

    // Extract Button A
    let button_a = lines[0]
        .strip_prefix("Button A: ")
        .and_then(|desc| {
            let parts: Vec<&str> = desc.split(", ").collect();
            let dx = parts[0].strip_prefix("X+").and_then(|x| x.parse::<isize>().ok())?;
            let dy = parts[1].strip_prefix("Y+").and_then(|y| y.parse::<isize>().ok())?;
            Some(Point{ x: dx + 0, y: dy + 0})
        })?;

    // Extract Button B
    let button_b = lines[1]
        .strip_prefix("Button B: ")
        .and_then(|desc| {
            let parts: Vec<&str> = desc.split(", ").collect();
            let dx = parts[0].strip_prefix("X+").and_then(|x| x.parse::<isize>().ok())?;
            let dy = parts[1].strip_prefix("Y+").and_then(|y| y.parse::<isize>().ok())?;
            Some(Point{ x: dx + 0, y: dy + 0})
        })?;

    // Extract Prize
    let prize = lines[2]
        .strip_prefix("Prize: ")
        .and_then(|desc| {
            let parts: Vec<&str> = desc.split(", ").collect();
            let x = parts[0].strip_prefix("X=").and_then(|x| x.parse::<isize>().ok())?;
            let y = parts[1].strip_prefix("Y=").and_then(|y| y.parse::<isize>().ok())?;
            Some(Point{ x: x + 0, y: y + 0})
        })?;
    Some(Machine { A: button_a, B: button_b, prize: prize })
}


fn find_press_combinations(m: Machine) -> Vec<(isize, isize)> {
    let x_target = m.prize.x;
    let y_target = m.prize.y;
 
    println!("Finding presses for target: ({}, {})", x_target, y_target);

    let mut found: Vec<(isize,isize)> = Vec::new();
    
    // Try all possible combinations of presses for Button A
    for n_a in 0..=x_target / m.A.x {
        let remaining_x = x_target - n_a * m.A.x;
        let remaining_y = y_target - n_a * m.A.y;

        // If the remaining distance is divisible by Button B's movement
        if remaining_x % m.B.x == 0 && remaining_y % m.B.y == 0 {
            let n_b_x = remaining_x / m.B.x;
            let n_b_y = remaining_y / m.B.y;

            if n_b_x == n_b_y && n_b_x >= 0 {
                println!("P1: Press Button A {} times, Button B {} times", n_a, n_b_x);
                found.push((n_a, n_b_x));
            }
        }
    }

    if found.len() == 0 {
        println!("P1: No valid combinations found.");
    }
    found
}

// There were no cases in my input where there were multiple solutions to any
// machine. I'm assuming that was a red herring so we only need to solve once
// Can't solve the machine for 10 trillion iterations so we need to
// be smarter - leaving the original part 1 solution find_press_combinations()
// redo more sensibly with linear equation using cramer's Rule
// https://en.wikipedia.org/wiki/Cramer%27s_rule
fn solve_machine( m: Machine ) -> Option<(isize, isize)> {
    // Determinant of the coefficient matrix
    let det_a = m.A.x * m.B.y - m.B.x * m.A.y;

    if det_a == 0 {
        // If determinant is zero, no unique solution exists
        return None;
    }

    // Determinants of replacement matrices
    let det_x = m.prize.x * m.B.y - m.B.x * m.prize.y;
    let det_y = m.A.x * m.prize.y - m.prize.x * m.A.y;

    // Solve for number of presses
    let n_a = det_x / det_a;
    let n_b = det_y / det_a;

    // Check if the solution makes sense
    if n_a >= 0 && n_b >= 0 && det_x % det_a == 0 && det_y % det_a == 0 {
        println!("P2: Press Button A {} times, Button B {} times", n_a, n_b);
        Some((n_a, n_b))
    } else {
        println!("P2: No valid combinations found.");
        None
    }
}

fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: isize = 0;
    let mut part2_answer: isize = 0;

    let mut lines_buffer: Vec<String> = Vec::new();
    let mut machines: Vec<Machine> = Vec::new();
    // Process each line
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            if let Some(machine) = parse_claw_machine(&lines_buffer) {
               machines.push(machine);
            }
            lines_buffer.clear();
        }
        else {
            lines_buffer.push(line);
        }
    }
    if !lines_buffer.is_empty() {
        if let Some(machine) = parse_claw_machine(&lines_buffer) {
            machines.push(machine);
        }
    }

    for mut m in machines {
        println!("{:?}", m);
        // part 1
        let combinations = find_press_combinations(m);
        let mut lowest_cost: isize = isize::MAX;
        for c in &combinations {
            let cost = (c.0 * 3) + c.1;
            if cost < lowest_cost {
                lowest_cost = cost;
            }
        }
        if !combinations.is_empty() && lowest_cost < isize::MAX {
            part1_answer += lowest_cost;
        }

        // part 2
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;
        if let Some(buttons) = solve_machine(m) {
            let cost = (buttons.0 * 3) + buttons.1;
            part2_answer += cost;
        }
    }
    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

