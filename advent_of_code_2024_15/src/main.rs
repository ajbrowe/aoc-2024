use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use std::collections::HashMap;
use std::collections::HashSet;
//use regex::Regex;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WideBox {
    x: usize,
    y: usize,
}

impl WideBox {
    fn points (self) -> (Point, Point) {
        (Point{ x: self.x, y: self.y}, Point{ x: self.x + 1, y: self.y })
    }
}

impl Point {
    fn to_wide_box (self) -> WideBox {
        WideBox{ x: self.x, y: self.y}
    }
}

// Define directions (N, S, E, W)
const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn parse_moves(lines: &Vec<String>) -> Vec<usize> {
    lines
        .iter()
        .flat_map(|line| line.chars().filter_map(|ch| match ch {
            '^' => Some(0),  // North
            'v' => Some(1),  // South
            '<' => Some(2),  // West
            '>' => Some(3),  // East
            _ => None,       // Ignore invalid characters
        }))
        .collect()
}


fn next_point( p: Point, m: usize, grid: &Vec<Vec<bool>>) -> Option<Point> {
    let dir = DIRECTIONS[m];
    let new_x = p.x as isize + dir.0;
    let new_y = p.y as isize + dir.1;
    if new_x < 0 || new_x >= grid[0].len() as isize || new_y < 0 || new_y >= grid.len() as isize {
        return None;
    }
    if grid[new_y as usize][new_x as usize] {
        return None;
    }
    Some(Point{ x: new_x as usize, y: new_y as usize })
}

fn can_move(p: Point, m: usize, grid: &Vec<Vec<bool>>, boxes: &HashSet<Point>) -> bool {

    if let Some(new_p) = next_point(p, m, grid) {
        if boxes.contains(&new_p) {
            return can_move(new_p, m, grid, boxes);
        }
        return true
    }
    false
}

fn move_box(p: Point, m: usize, grid: &Vec<Vec<bool>>, boxes: &mut HashSet<Point>) {
    if let Some(new_p) = next_point(p, m, grid) {
        if boxes.contains(&new_p) {
            move_box(new_p, m, grid, boxes);
        }
    // recursive calls above will have cleared the space of other boxes
    // so we can move this box now
    boxes.remove(&p);
        if !boxes.insert(new_p) {
            println!("moving box at {:?} to {:?} but it was already there", p, new_p);
        }
    }
    else {
        println!("couldn't move a box at {:?} to {:?}", p, m);
    }
}

fn find_box(p: Point, boxes: &HashSet<WideBox>) -> Option<WideBox> {
    let mut wb: WideBox = p.to_wide_box();
    if boxes.contains(&wb) {
        if let Some(b) = boxes.get(&wb) {
            return Some(*b);
        }
    }
    wb.x -= 1;
    if boxes.contains(&wb) {
        if let Some(b) = boxes.get(&wb) {
            return Some(*b);
        }
    }
    None
}
fn can_move_wide(p: Point, m: usize, grid: &Vec<Vec<bool>>, boxes: &HashSet<WideBox>) -> bool {

    if let Some(new_p) = next_point(p, m, grid) {
        if let Some(wb) = find_box(new_p, boxes) {
            // Wide box has two points 
            // AB
            let (p1, p2) = wb.points();
            // if we are moving North or South
            // ^^ 
            // AB or
            // vv
            if m <= 1 {
                return can_move_wide(p1, m, grid, boxes) && can_move_wide(p2, m, grid, boxes);
            }
            // if we are moving West
            // <AB
            else if m == 2 {
                return can_move_wide(p1, m, grid, boxes);
            }
            // or we are moving East
            //  AB>
            else {
                return can_move_wide(p2, m, grid, boxes);
            }
        }
        return true
    }
    false
}

fn move_wide_box(p: Point, m: usize, grid: &Vec<Vec<bool>>, boxes: &mut HashSet<WideBox>) {
    let dir = vec!["^","v","<",">"];
    if let Some(wb) = find_box(p, boxes) {
        // Wide box has two points 
        // AB A= p1, B=p2
        let (p1, p2) = wb.points();
        let mut check_points: Vec<Point> = Vec::new();
        // if we are moving North or South check p1 and p2
        if m <= 1 {
            check_points.push(p1);
            check_points.push(p2);
        }
        // if we are moving West check p1
        else if m == 2 {
            check_points.push(p1);
        }
        // otherwise we are moving East so check p2
        else {
            check_points.push(p2);
        }

        for bp in check_points {
            if let Some(new_p) = next_point(bp, m, grid) {
                move_wide_box(new_p, m, grid, boxes);
            }
        }

        // recursive calls above will have cleared the space of other boxes
        // so we can move this box now
        boxes.remove(&wb);
        if let Some(new_p) = next_point(p1, m, grid) {
            if !boxes.insert(new_p.to_wide_box()) {
                println!("ERROR!: moving box at {:?} to {:?} but it was already there", p1, new_p);
            }
        }
        else {
            println!("ERROR!: Failed trying to move a box at {:?} to {}", p1, dir[m]);
        }
    }
}

fn render_wide_grid(robot: Point, grid: &Vec<Vec<bool>>, boxes: &HashSet<WideBox>, counter: usize) {
    println!("Grid {}", counter);
    for (row_idx, row) in grid.iter().enumerate() {
        let mut box_present: bool = false;
        for (col_idx, solid) in row.iter().enumerate() {
            if *solid {
                print!("#");
                continue;
            }
            if box_present {
                print!("]");
                box_present = false;
                continue;
            }
            if robot.x == col_idx && robot.y == row_idx {
                print!("@");
                continue;
            }
            let t_wb = WideBox{ x: col_idx, y: row_idx };
            if boxes.contains(&t_wb) {
                print!("[");
                box_present = true;
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
    //let path = Path::new("sample3");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: usize = 0;
    let mut part2_answer: usize = 0;

    // Process each line
    let mut reading_moves: bool = false;
    let mut grid_lines: Vec<String> = Vec::new();
    let mut move_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            reading_moves = true;
            continue;
        }
        if reading_moves {
            move_lines.push(line);
        }
        else {
            grid_lines.push(line);
        }
    }
    let mut boxes: HashSet<Point> = HashSet::new();
    let mut wide_boxes: HashSet<WideBox> = HashSet::new();
    let mut robot: Point = Point{ x: 0, y: 0};
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for (row_idx, line) in grid_lines.iter().enumerate() {
        let mut row: Vec<bool> = Vec::new();
        for (col_idx, ch) in line.chars().enumerate() {
            row.push(ch == '#');
            if ch == '@' {
                robot.x = col_idx;
                robot.y = row_idx;
            }
            else if ch == 'O' {
                boxes.insert(Point{x: col_idx, y: row_idx});
                wide_boxes.insert(WideBox{x: col_idx * 2, y: row_idx});
            }
        }
        grid.push(row);
    }

    let mut p2_robot: Point = Point{ x: robot.x * 2, y: robot.y };

    let moves: Vec<usize> = parse_moves(&move_lines);
    for &m in moves.iter() {
        if can_move(robot, m, &grid, &boxes) {
            if let Some(new_p) = next_point(robot, m, &grid) {
                if boxes.contains(&new_p) {
                    move_box(new_p, m, &grid, &mut boxes);
                }
                robot.x = new_p.x;
                robot.y = new_p.y;
            }
        }
    }

    for b in boxes.iter() {
        part1_answer += (b.y * 100) + b.x;
    }

    let mut wide_grid: Vec<Vec<bool>> = Vec::new();
    for row in grid {
        let mut wide_row: Vec<bool> = Vec::new();
        for col in row {
            wide_row.push(col);
            wide_row.push(col);
        }
        wide_grid.push(wide_row);
    }

    let mut counter: usize = 0;
    render_wide_grid(p2_robot, &wide_grid, &wide_boxes, counter);
    for &m in moves.iter() {
        counter += 1;
        if can_move_wide(p2_robot, m, &wide_grid, &wide_boxes) {
            if let Some(new_p) = next_point(p2_robot, m, &wide_grid) {
                if let Some(wb) = find_box(new_p, &wide_boxes) {
                    move_wide_box(wb.points().0, m, &wide_grid, &mut wide_boxes);
                    //move_wide_box(wb.points().1, m, &wide_grid, &mut wide_boxes);
                }
                p2_robot.x = new_p.x;
                p2_robot.y = new_p.y;
            }
        }
       // render_wide_grid(p2_robot, &wide_grid, &wide_boxes, counter);
    }

    render_wide_grid(p2_robot, &wide_grid, &wide_boxes, counter);
    for b in wide_boxes.iter() {
        part2_answer += (b.y * 100) + b.x;
    }



    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

