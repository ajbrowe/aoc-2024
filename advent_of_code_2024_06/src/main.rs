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
struct Guard {
    pos: Point,
    dx: i64,
    dy: i64,
}

impl Guard {
    fn turn_right(&mut self) {
        let (new_dx, new_dy) = match(self.dx, self.dy) {
            (-1, 0) => (0, -1), // West -> North
            (0, -1) => (1, 0),  // North -> East
            (1, 0)  => (0, 1),  // East -> South
            (0, 1) => (-1, 0),  // South -> West
            _ => (self.dx, self.dy), // Default case (invalid state, should not happen)
        };
        self.dx = new_dx;
        self.dy = new_dy;
    }
 
    fn leaving_grid(&mut self, grid: &[Vec<char>]) -> bool {
        let height = grid.len() as i64 ;
        let width  = grid[0].len() as i64;
        let next_x = self.next_x();
        let next_y = self.next_y();

        return next_x < 0 || next_x >= width || next_y < 0 || next_y >= height;
    }

    fn next_x(&mut self) -> i64 {
        let current_x = self.pos.x as i64;
        let next_x: i64 = current_x + self.dx;
        return next_x;
    }

    fn next_y(&mut self) -> i64 {
        let current_y = self.pos.y as i64;
        let next_y: i64 = current_y + self.dy;
        return next_y;
    }

    fn can_move(&mut self, grid: &[Vec<char>]) -> bool {
        let next_x = self.next_x();
        let next_y = self.next_y();

        if next_x < 0 || next_y < 0 {
            return true; // Going off grid is allowed
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if next_y >= grid.len() || next_x >= grid[0].len() {
            return true; // going off grid is allowed
        }

        // check grid is clear at the next position
        return grid[next_y][next_x] != '#';

    }

    fn can_move_obstacle(&mut self, grid: &[Vec<char>], obstacle: &Point) -> bool {

        let next_x = self.next_x();
        let next_y = self.next_y();

        if next_x < 0 || next_y < 0 {
            return true; // Going off grid is allowed
        }

        let next_x = next_x as usize;
        let next_y = next_y as usize;

        if next_y >= grid.len() || next_x >= grid[0].len() {
            return true; // going off grid is allowed
        }

        let next_pos = Point {
            x: next_x,
            y: next_y,
        };
        if next_pos == *obstacle {
            return false;
        }
        // check grid is clear at the next position
        return grid[next_y][next_x] != '#';

    }


    fn move_step(&mut self) {
        self.pos.x = self.next_x() as usize;
        self.pos.y = self.next_y() as usize;
    }


}


fn find_guard(grid: &[Vec<char>]) -> Option<Guard> {
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&c| c == '^') {
            return Some(Guard {
                pos: Point {
                    x: col_idx,
                    y: row_idx,
                },
                dx: 0,
                dy: -1,
            });
        }
    }
    None
}

fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: u64 = 0;
    let mut part2_answer: u64 = 0;


    // Process each line
    let grid: Vec<Vec<char>> = reader.lines().map(|line| line.expect("Could not read line").chars().collect()).collect();

    let height: usize = grid.len();
    let width: usize = grid[0].len();
    let mut visited: HashMap<Point, u64> = HashMap::new();

    if let Some(mut guard) = find_guard(&grid) {
        visited.insert(guard.pos, 1);
        while !guard.leaving_grid(&grid) {
            if guard.can_move(&grid) {
                guard.move_step();
                *visited.entry(guard.pos).or_insert(0) += 1;
            } else {
                guard.turn_right();
            }
        }
    }
    part1_answer = visited.len() as u64;
    

    // Get the guard back at the original location
    if let Some(original_guard) = find_guard(&grid) {
        for obstacle_position in visited.keys() {
            let mut travelled: HashMap<Guard, u64> = HashMap::new();
            let mut guard = original_guard.clone();
            travelled.insert(guard, 1);

            // Don't add an obstacle where the guard starts
            if *obstacle_position == original_guard.pos {
                continue;
            }
            while !guard.leaving_grid(&grid) {
                if guard.can_move_obstacle(&grid, &obstacle_position) {
                    guard.move_step();
                    if travelled.contains_key(&guard) {
                        part2_answer += 1;
                        break;
                    }
                    travelled.insert(guard, 1);
                } else {
                    guard.turn_right();
                }
            }
        }
    }

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

