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
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (0, -1), (-1, 0)];

// DFS function for flood fill
fn dfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: Point,
    current_char: char,
    region: &mut Vec<Point>,
) {
    let x = start.x;
    let y = start.y;

    // Boundary and visit checks
    if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
        return;
    }
    if visited[y as usize][x as usize] || grid[y as usize][x as usize] != current_char {
        return;
    }

    // Mark as visited and add to region
    visited[y as usize][x as usize] = true;
    region.push(Point { x: x, y: y});

    // Recur in all directions
    for (dx, dy) in DIRECTIONS.iter() {
        dfs(
            grid,
            visited,
            Point {
                x: (x + dx),
                y: (y + dy),
            },
            current_char,
            region,
        );
    }
}

fn find_regions(grid: &Vec<Vec<char>>) -> Vec<Vec<Point>> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut regions: Vec<Vec<Point>> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited[y][x] {
                let mut region: Vec<Point> = Vec::new();
                dfs(&grid, &mut visited, Point { x: x as isize, y: y as isize }, grid[y][x], &mut region);

                if !region.is_empty() {
                    regions.push(region);
                }
            }
        }
    }
    regions
}

fn find_perimeter(grid: &Vec<Vec<char>>, p: &Point) -> usize {

    let mut perimeters: usize = 0;
    let identifier: char = grid[p.y as usize][p.x as usize];
    for (dx, dy) in DIRECTIONS.iter() {
        let new_x = p.x + dx;
        let new_y = p.y + dy;
        if new_x < 0 || new_x >= grid[0].len() as isize || new_y < 0 || new_y >= grid.len() as isize {
            perimeters += 1;
        }
        else if grid[new_y as usize][new_x as usize] != identifier {
            perimeters += 1;
        }
    }
    perimeters
}

fn count_sides( region: &Vec<Point>) -> usize {
    let mut sides: usize = 0;

    for p in region {
        // Check convex corner.
        // ?B
        // BA
        if !region.contains(&Point{ x: p.x, y: p.y - 1 }) && !region.contains(&Point{ x: p.x -1, y: p.y }) {
            sides += 1;
        }
        // BA
        // AA
        else if region.contains(&Point{ x: p.x, y: p.y - 1}) && region.contains(&Point{ x: p.x -1, y: p.y}) {
            if !region.contains(&Point{ x: p.x - 1, y: p.y - 1}) {
                sides += 1;
            }
        }
        // Check convex corner.
        // B?
        // AB
        if !region.contains(&Point{ x: p.x, y: p.y - 1 }) && !region.contains(&Point{ x: p.x + 1, y: p.y }) {
            sides += 1;
        }
        // AB
        // AA
        else if region.contains(&Point{ x: p.x, y: p.y - 1}) && region.contains(&Point{ x: p.x + 1, y: p.y}) {
            if !region.contains(&Point{ x: p.x + 1, y: p.y - 1}) {
                sides += 1;
            }
        }
        // Check convex corner.
        // AB
        // B?
        if !region.contains(&Point{ x: p.x, y: p.y + 1 }) && !region.contains(&Point{ x: p.x + 1, y: p.y }) {
            sides += 1;
        }
        // AA
        // AB
        else if region.contains(&Point{ x: p.x, y: p.y + 1}) && region.contains(&Point{ x: p.x + 1, y: p.y}) {
            if !region.contains(&Point{ x: p.x + 1, y: p.y + 1}) {
                sides += 1;
            }
        }
        // Check convex corner.
        // BA
        // ?B
        if !region.contains(&Point{ x: p.x, y: p.y + 1 }) && !region.contains(&Point{ x: p.x -1, y: p.y }) {
            sides += 1;
        }
        // AA
        // BA
        else if region.contains(&Point{ x: p.x, y: p.y + 1}) && region.contains(&Point{ x: p.x - 1, y: p.y}) {
            if !region.contains(&Point{ x: p.x - 1, y: p.y + 1}) {
                sides += 1;
            }
        }}


    sides
}


fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample1");
    //let path = Path::new("sample2");
    //let path = Path::new("sample3");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: usize = 0;
    let mut part2_answer: usize = 0;

    // Process each line
    let grid: Vec<Vec<char>> = reader.lines().map(|line| line.expect("Could not read line").chars().collect()).collect();

    let regions = find_regions(&grid);
    for region in &regions {
         let area: usize = region.len();
         let mut perimeter: usize = 0;
         for p in region {
             perimeter += find_perimeter(&grid, p);
         }
         part1_answer += area * perimeter;
         part2_answer += area * count_sides(region);
    }

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

