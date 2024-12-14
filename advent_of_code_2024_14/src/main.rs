use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
//use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn parse_point(s: &str, prefix: &str) -> Option<Point> {
    if let Some(coords) = s.strip_prefix(prefix) {
        let parts: Vec<&str> = coords.split(',').collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<isize>().unwrap_or(0);
            let y = parts[1].parse::<isize>().unwrap_or(0);
            return Some(Point{ x, y });
        }
    }
    None
}

fn predict_position( r: &Robot, time: isize, space: &Point ) -> Point {
    let max_x = space.x;
    let max_y = space.y;

    // euclidean_modulo
    let new_x = (((r.pos.x + (r.vel.x * time)) % max_x) + max_x) % max_x;
    let new_y = (((r.pos.y + (r.vel.y * time)) % max_y) + max_y) % max_y;
    return Point{ x: new_x, y: new_y };
}

fn quadrant( p: &Point, space: &Point ) -> Option<usize> {
    let mid_x = space.x / 2;
    let mid_y = space.y / 2;

    // NW Quadrant
    if p.x < mid_x && p.y < mid_y {
        return Some(0);
    }
    // NE Quadrant
    if p.x > mid_x && p.y < mid_y {
        return Some(1);
    }
     // SE Quadrant
    if p.x > mid_x && p.y > mid_y {
        return Some(2);
    }
    // SW Quadrant
    if p.x < mid_x && p.y > mid_y {
        return Some(3);
    }

    None
}

fn render_at (robots: &Vec<Robot>, space: &Point, time: isize) {
    println!("Robot Map at {} seconds", time);

    let mut robot_positions: HashMap<Point, usize> = HashMap::new();
    for r in robots {
        let p = predict_position(&r, time, space);
        *robot_positions.entry(p).or_insert(0) += 1;
    }

    for y in 0..space.y {
        for x in 0..space.x {
            let p: Point = Point{x: x, y: y};
            if robot_positions.contains_key(&p) {
                print!("X");
            }
            else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn robots_in_distinct_positions (robots: &Vec<Robot>, space: &Point, time: isize) -> bool {

    let mut robot_positions: HashMap<Point, usize> = HashMap::new();
    for r in robots {
        let p = predict_position(&r, time, space);
        *robot_positions.entry(p).or_insert(0) += 1;
    }
    if robots.len() == robot_positions.len() {
        return true;
    }
    false
}

fn main() -> io::Result<()> {

    // Open the file
    //let filename = "sample";
    let filename = "input";
    //let path = Path::new("input");
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);


    let space = if filename == "input" { Point{ x: 101, y: 103 } } else { Point{ x: 11, y: 7 } };

    let mut robots: Vec<Robot> = Vec::new();
    // Process each line
    for line in reader.lines() {
        let line = line?;
        let details: Vec<&str> = line.split_whitespace().collect();
        if let Some(p) = parse_point(details[0], "p=") {
            if let Some(v) = parse_point(details[1], "v=") {
                robots.push(Robot{ pos: p, vel: v });
            }
        }
        
    }
    let mut quadrant_count: HashMap<usize,usize> = HashMap::new();
    for r in &robots {
        let p = predict_position(&r, 100, &space);
        println!("Robot {:?} at pos {:?} after 100 seconds", r, p);
        if let Some(q) = quadrant(&p, &space) {
            println!("In Quadrant {}", q);
            *quadrant_count.entry(q).or_insert(0) += 1;
        }
        else {
            println!("In middle");
        }
    }

    let part1_answer: usize = quadrant_count.values().cloned().product();
    println!("Quadrants: {:?}", quadrant_count);
    
    let mut i: isize = 1;
    while !robots_in_distinct_positions(&robots, &space, i) && i < 100000 {
        i += 1;
    }
    render_at(&robots, &space, i);
    let part2_answer = i;
    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

