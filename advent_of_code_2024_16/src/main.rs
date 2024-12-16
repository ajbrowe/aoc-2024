use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
//use regex::Regex;
use std::collections::{BinaryHeap, HashMap, HashSet};
//use std::cmp::{Ordering, Reverse};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Clone, Copy)]
struct State {
    point: Point,
    cost: usize,
    facing: usize, // 0 = North, 1 = East, 2 = South, 3 = West
}
// Implement ordering based on cost
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse to make BinaryHeap a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

// Direction vectors (North, East, South, West)
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];


fn backtrack_all_paths(
    end: Point,
    facing: usize,
    predecessors: &HashMap<(Point, usize), Vec<(Point, usize)>>,
    visited: &mut HashSet<Point>,
) {
    let mut stack = vec![(end, facing)];
    let mut visited_states: HashSet<(Point, usize)> = HashSet::new();

    while let Some((current, dir)) = stack.pop() {
        if !visited_states.insert((current, dir)) {
            continue; // Skip already visited states
        }

        visited.insert(current); // Record the point as visited

        if let Some(parents) = predecessors.get(&(current, dir)) {
            for &(prev_point, prev_facing) in parents {
                stack.push((prev_point, prev_facing));
            }
        }
    }
}

fn find_all_lowest_cost_paths(grid: &Vec<Vec<char>>, start: Point, end: Point) -> (usize, usize) {
    let mut heap = BinaryHeap::new();
    let mut min_cost: HashMap<(Point, usize), usize> = HashMap::new();
    let mut predecessors: HashMap<(Point, usize), Vec<(Point, usize)>> = HashMap::new();

    // Start facing East (1)
    heap.push(State {
        point: start,
        cost: 0,
        facing: 1,
    });
    min_cost.insert((start, 1), 0);

    let mut best_cost = usize::MAX;

    while let Some(current) = heap.pop() {
        if current.point == end {
            best_cost = current.cost.min(best_cost);
        }

        if current.cost > best_cost {
            continue; // No need to explore worse paths
        }

        // Explore all possible moves (forward + turns)
        for (dir, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let next_point = Point {
                x: (current.point.x as isize + dx) as usize,
                y: (current.point.y as isize + dy) as usize,
            };

            // Check if next point is within bounds and not a wall
            if next_point.y >= grid.len() || next_point.x >= grid[0].len() || grid[next_point.y][next_point.x] == '#' {
                continue;
            }

            // Calculate movement cost
            let mut next_cost = current.cost;

            if dir == current.facing {
                // Moving forward
                next_cost += 1;
            } else {
                // Turning cost + moving forward
                let turn_cost = ((4 + dir as isize - current.facing as isize) % 4).min(
                    ((4 + current.facing as isize - dir as isize) % 4),
                );
                next_cost += 1000 * turn_cost as usize + 1;
            }

            // If we found a better way - clear any previous paths stored to this node
            if next_cost < *min_cost.get(&(next_point, dir)).unwrap_or(&usize::MAX) {
                min_cost.insert((next_point, dir), next_cost);
                predecessors.entry((next_point, dir))
                    .or_insert_with(Vec::new)
                    .clear(); // Clear old paths if new best cost is found
            }
            // if we found a way that matches the best known path then store
            if next_cost == *min_cost.get(&(next_point, dir)).unwrap_or(&usize::MAX) {
                predecessors.entry((next_point, dir))
                    .or_insert_with(Vec::new)
                    .push((current.point, current.facing));
                heap.push(State {
                    point: next_point,
                    cost: next_cost,
                    facing: dir,
                });
            }

        }
    }

    // Collect all unique points from paths
    let mut visited_points: HashSet<Point> = HashSet::new();
    for dir in 0..4 {
        if *min_cost.get(&(end, dir)).unwrap_or(&usize::MAX) == best_cost {
            backtrack_all_paths(end, dir, &predecessors, &mut visited_points);
        }
    }

    render_grid_visited( grid, &visited_points );
    (best_cost, visited_points.len())
}

fn render_grid_visited(grid: &Vec<Vec<char>>, visited_points: &HashSet<Point>) {
    println!("Grid Visited");
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, ch) in row.iter().enumerate() {
            if *ch == '#' {
                print!("#");
                continue;
            }
            let p = Point{ x: col_idx, y: row_idx };
            if visited_points.contains(&p) {
                print!("O");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}


fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample2");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines().map(|line| line.expect("Could not read line").chars().collect()).collect();

    let mut start: Point = Point{x: 0, y: 0};
    let mut end: Point = Point{x: 0, y: 0};
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                start.x = col_idx;
                start.y = row_idx;
            }
            if *ch == 'E' {
                end.x = col_idx;
                end.y = row_idx;
            }
        }
    }

    let (part1_answer, part2_answer) = find_all_lowest_cost_paths(&grid, start, end);

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

