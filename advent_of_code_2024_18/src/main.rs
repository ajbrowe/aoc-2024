use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{BinaryHeap, HashMap};
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


fn find_lowest_cost_path(grid: &Vec<Vec<char>>, start: Point, end: Point) -> usize {
    let mut heap = BinaryHeap::new();
    let mut min_cost: HashMap<Point, usize> = HashMap::new();

    heap.push(State {
        point: start,
        cost: 0,
    });
    min_cost.insert(start, 0);

    let mut best_cost = usize::MAX;

    while let Some(current) = heap.pop() {
        if current.point == end {
            best_cost = current.cost.min(best_cost);
            break;
        }

        if current.cost > best_cost {
            continue; // No need to explore worse paths
        }

        // Explore all possible moves (forward + turns)
        for (dx, dy) in DIRECTIONS.iter() {
            let next_y = current.point.y as isize + dy;
            let next_x = current.point.x as isize + dx;
            let max_y = grid.len() as isize;
            let max_x = grid[0].len() as isize;
            if next_y < 0 || next_y >= max_y || next_x < 0 || next_x >= max_x || grid[next_y as usize][next_x as usize] == '#' {
                continue;
            }

            let next_point: Point = Point{x: next_x as usize, y: next_y as usize};

            // Calculate movement cost
            let next_cost = current.cost + 1;

            // If we found a better way - clear any previous paths stored to this node
            if next_cost < *min_cost.get(&next_point).unwrap_or(&usize::MAX) {
                min_cost.insert(next_point, next_cost);
                heap.push(State {
                    point: next_point,
                    cost: next_cost,
                });
            }

        }
    }

    best_cost
}

fn build_grid(locations: &Vec<Point>, time: usize, max: Point) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..=max.y {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..=max.x {
            row.push('.');
        }
        grid.push(row);
    }
    for &l in locations[0..time].iter() {
        grid[l.y][l.x] = '#';
    }
    grid
}

fn find_point_of_no_return(locations: &Vec<Point>, end: Point, good: usize, bad: usize) -> usize {
    // If we no longer have anywhere else to check then we found the first bad
    if good == bad - 1 {
        return bad;
    }

    let mid = good + ((bad - good) / 2);

    let grid = build_grid(locations, mid, end);
    let best_path: usize = find_lowest_cost_path(&grid, Point{x: 0, y: 0}, end);
    if best_path < usize::MAX {
        return find_point_of_no_return(locations, end, mid, bad);
    }
    find_point_of_no_return(locations, end, good, mid)
}


fn main() -> io::Result<()> {

    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample2");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut locations: Vec<Point> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some((x_str, y_str)) = line.split_once(',') {
            if let (Ok(x), Ok(y)) = (x_str.trim().parse::<usize>(), y_str.trim().parse::<usize>()) {
                locations.push(Point{ x, y });
            }
        }
    }


    let start: Point = Point{x: 0, y: 0};
    let end: Point = Point{x: 70, y: 70};
    //let mut end: Point = Point{x: 6, y: 6};

    //let mut grid: Vec<Vec<char>> = build_grid(&locations, 12, end);
    let grid: Vec<Vec<char>> = build_grid(&locations, 1024, end);

    let part1_answer = find_lowest_cost_path(&grid, start, end);

    let point_of_no_return = find_point_of_no_return(&locations, end, 1024, locations.len());
    //let point_of_no_return = find_point_of_no_return(&locations, end, 12, locations.len());
    let bad_point = locations[point_of_no_return - 1];

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {},{}", bad_point.x, bad_point.y);

    Ok(())
}

