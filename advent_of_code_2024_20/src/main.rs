use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance (self, other: &Point) -> usize {
        let distance = (self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs();
        distance as usize
    }
}

// Direction vectors (North, East, South, West)
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn bfs(grid: &Vec<Vec<char>>, start: Point, end: Point) -> Vec<Point> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut predecessors: HashMap<Point, Point> = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            let mut path = vec![end];
            let mut backtrack = end;
            while let Some(&prev) = predecessors.get(&backtrack) {
                path.push(prev);
                backtrack = prev;
            }
            path.reverse();
            return path;
        }

        for &(dx, dy) in DIRECTIONS.iter() {
            let next = Point {
                x: (current.x as isize + dx) as usize,
                y: (current.y as isize + dy) as usize,
            };

            if next.y < grid.len()
                && next.x < grid[0].len()
                && grid[next.y][next.x] != '#'
                && !visited.contains(&next)
            {
                visited.insert(next);
                predecessors.insert(next, current);
                queue.push_back(next);
            }
        }
    }
    vec![]
}

fn find_candidate_removals(path: &Vec<Point>, min_saving: usize, cheat_distance: usize) -> HashSet<(Point,Point)> {
    let mut seen: HashSet<(Point,Point)> = HashSet::new();
    for (i, &p1) in path.iter().enumerate() {
        for (j, &p2) in path.iter().enumerate().skip(i + 2) {
            let distance = p1.distance(&p2);
            let saving = (j -i) - distance;
            if distance <= cheat_distance  && saving >= min_saving {
                seen.insert((p1,p2));
            }
        }
    }
    seen
}


fn evaluate_cheats(grid: Vec<Vec<char>>, start: Point, end: Point, min_saving: usize, cheat_distance: usize) -> usize{
    let original_path = bfs(&grid, start, end);

    let candidates = find_candidate_removals(&original_path, min_saving, cheat_distance);
    candidates.len()
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

    let part1_answer = evaluate_cheats(grid.clone(), start, end, 100, 2);
    let part2_answer = evaluate_cheats(grid.clone(), start, end, 100, 20);

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

