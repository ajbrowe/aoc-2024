use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antenna {
    position: Point,
    frequency: char,
}

impl Antenna {
    fn distance_to(self, other: &Antenna) -> Distance {
        Distance {
            x: self.position.x as isize - other.position.x as isize,
            y: self.position.y as isize - other.position.y as isize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Distance {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Antinode {
    position: Point,
    a: Antenna,
    b: Antenna,
}

fn antinode_position(a: &Point, d: &Distance, max: &Point) -> Option<Point> {
    let new_x = a.x as isize + d.x;
    let new_y = a.y as isize + d.y;
    if new_x < 0 || new_x > max.x as isize || new_y < 0 || new_y > max.y as isize {
        return None
    }
    return Some(Point{ x: new_x as usize, y: new_y as usize });
}

// Function to print the grid
fn print_grid(
    max_x: usize,
    max_y: usize,
    antennas: &HashMap<Point, char>,
    antinodes: &HashMap<Point, usize>,
) {
    println!("\nGrid:");
    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Point { x, y };
            
            // Print antenna if it exists
            if let Some(&frequency) = antennas.get(&pos) {
                print!("{}", frequency);
            } 
            // Print antinode if no antenna exists
            else if antinodes.contains_key(&pos) {
                print!("#");
            } 
            // Print empty space if nothing exists
            else {
                print!(".");
            }
        }
        println!(); // Newline after each row
    }
}

fn find_antinodes_p2 (a: &Point, d: &Distance, max: &Point) -> Option<Vec<Point>> {
    let mut points: Vec<Point> = Vec::new();

    let mut pos = *a;
    points.push(pos);
    let mut distance = *d;

    while let Some(antinode_pos) = antinode_position(&pos, &distance, max) {
        points.push(antinode_pos);
        pos = antinode_pos;
    }

    // reverse distance and search other direction
    distance = Distance{ x: d.x * -1, y: d.y * -1};
    // Go back to original position
    pos = *a;
    while let Some(antinode_pos) = antinode_position(&pos, &distance, max) {
        points.push(antinode_pos);
        pos = antinode_pos;
    }
    if points.len() > 0 {
        return Some(points);
    }
    None

}

fn main() -> io::Result<()> {
    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut antinodes: Vec<Antinode> = Vec::new();
    let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();

    let mut antenna_locations: HashMap<Point, char> = HashMap::new();
    let mut antinode_locations: HashMap<Point, usize> = HashMap::new();
    let mut antinode_locations_p2: HashMap<Point, usize> = HashMap::new();

    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    // Process each line
    for (row_idx, line) in reader.lines().enumerate() {
        let line = line?;
        max_y = row_idx;

        for (col_idx, ch) in line.chars().enumerate() {
            // update max_x
            if col_idx > max_x {
                max_x = col_idx;
            }

            if ch != '.' {
                let antenna = Antenna {
                    position: Point { x: col_idx, y: row_idx },
                    frequency : ch,
                };
                antenna_map.entry(ch).or_insert_with(Vec::new).push(antenna);
                antenna_locations.insert(antenna.position, ch);
            }
        }
    }

    let max_pos: Point = Point { x: max_x, y: max_y };

    for (_frequency, antennas) in &mut antenna_map {
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a = &antennas[i];
                let b = &antennas[j];
                let d = &a.distance_to(b);
                if let Some(antinode_pos) = antinode_position(&a.position, &d, &max_pos) {
                    antinodes.push(Antinode{ position: antinode_pos, a: a.clone(), b: b.clone() });
                }
                let e = b.distance_to(a);
                if let Some(antinode_pos) = antinode_position(&b.position, &e, &max_pos) {
                    antinodes.push(Antinode{ position: antinode_pos, a: b.clone(), b: a.clone() });
                }
                
                if let Some(antinode_points) = find_antinodes_p2(&a.position, &d, &max_pos) {
                    for point in antinode_points {
                        *antinode_locations_p2.entry(point).or_insert(0) += 1;
                    }
                }

            }
        }
    }

    for an in &antinodes {
        *antinode_locations.entry(an.position).or_insert(0) += 1;
    }

    print_grid(max_x, max_y, &antenna_locations, &antinode_locations_p2);
    let part1_answer = antinode_locations.len();
    let part2_answer = antinode_locations_p2.len();
    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

