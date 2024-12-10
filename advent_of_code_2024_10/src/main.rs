use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Define directions (N, S, E, W)
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample2");
    //let path = Path::new("sample3");
    //let path = Path::new("sample4");
    //let path = Path::new("sample5");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut part1_answer: usize = 0;

    let mut part2_answer: usize = 0;


    let grid: Vec<Vec<usize>> = reader.lines().map(|line| {
        line.expect("Could not read line")
            .chars()
            .map(|ch| {
                if ch == '.' {
                    return 1000 as usize;
                }
                ch.to_digit(10).expect("Not a digit") as usize
            })
            .collect()
    }).collect();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == 0 {
                let visited1 = &mut vec![vec![false; grid[0].len()]; grid.len()];
                part1_answer += dfs(&grid, col_idx as isize, row_idx as isize, *col as isize - 1, visited1, false);
                let visited2 = &mut vec![vec![false; grid[0].len()]; grid.len()];
                part2_answer += dfs(&grid, col_idx as isize, row_idx as isize, *col as isize - 1, visited2, true);
            }
        }
    }


    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

// Depth-First Search Function
fn dfs(
    grid: &Vec<Vec<usize>>,
    x: isize,
    y: isize,
    current_value: isize,
    visited: &mut Vec<Vec<bool>>,
    p2: bool
) -> usize {
    // Check bounds
    if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
        return 0;
    }

    // Check if already visited or if value isn't one increment higher
    let value = grid[y as usize][x as usize] as isize;
    if visited[y as usize][x as usize] || value != current_value + 1 {
        return 0;
    }

    // Mark as visited
    visited[y as usize][x as usize] = true;

    // If we reached 9, we found a valid path
    if value == 9 {
        if p2 {
            visited[y as usize][x as usize] = false;
        }
        return 1;
    }

    // Explore all four directions
    let mut paths = 0;
    for (dx, dy) in DIRECTIONS.iter() {
        paths += dfs(grid, x + dx, y + dy, value as isize, visited, p2);
    }

    // Backtrack
    visited[y as usize][x as usize] = false;

    paths
}
