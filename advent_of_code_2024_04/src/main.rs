use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn count_occurrences(text: &str, word: &str) -> usize {
    let re = Regex::new(&regex::escape(word)).unwrap(); // Escape the word in case it has special characters
    re.find_iter(text).count()
}

fn search_horizontal_and_vertical(grid: &[Vec<char>], word: &str) -> usize {
    let reverse_word: String = word.chars().rev().collect();
    let mut count = 0;

    // Horizontal search
    for row in grid {
        let row_str: String = row.iter().collect();
        count += count_occurrences(&row_str, word);
        count += count_occurrences(&row_str, &reverse_word);
    }

    // Vertical search
    let n = grid.len();
    let m = grid[0].len();
    for col in 0..m {
        let col_str: String = (0..n).map(|row| grid[row][col]).collect();
        count += count_occurrences(&col_str, word);
        count += count_occurrences(&col_str, &reverse_word);
    }

    count
}
fn search_diagonals(grid: &[Vec<char>], word: &str) -> usize {
    let reverse_word: String = word.chars().rev().collect();
    let mut count = 0;
    let n = grid.len();
    let m = grid[0].len();

    // Top-left to bottom-right
    for d in 0..(n + m - 1) {
        let mut diagonal: Vec<char> = Vec::new();
        for i in 0..=d {
            let j = d - i;
            if i < n && j < m {
                diagonal.push(grid[i][j]);
            }
        }
        let diag_str: String = diagonal.iter().collect();
        count += count_occurrences(&diag_str, word);
        count += count_occurrences(&diag_str, &reverse_word);
    }

    // Top-right to bottom-left
    for d in 0..(n + m - 1) {
        let mut diagonal: Vec<char> = Vec::new();
        for i in 0..=d {
            if let Some(j) = m.checked_sub(1 + d - i) {
                if i < n && j < m {
                    diagonal.push(grid[i][j]);
                }
            }
        }
        let diag_str: String = diagonal.iter().collect();
        count += count_occurrences(&diag_str, word);
        count += count_occurrences(&diag_str, &reverse_word);
    }

    count
}

fn search_xmas_patterns(grid: &[Vec<char>]) -> usize {
    let mut count = 0;
    let n = grid.len();
    let m = grid[0].len();

    // Check each cell in the grid
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] == 'A' {
                // Check if diagonals form X-MAS
                if is_xmas_pattern(grid, i, j) {
                    count += 1;
                }
            }
        }
    }

    count
}

// Helper function to check the X-MAS pattern at a given position
fn is_xmas_pattern(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    // Extract diagonals
    let top_left = grid[i - 1][j - 1];
    let top_right = grid[i - 1][j + 1];
    let bottom_left = grid[i + 1][j - 1];
    let bottom_right = grid[i + 1][j + 1];

    // Check the diagonals
    let diagonal1 = [top_left, grid[i][j], bottom_right]; // Top-left to bottom-right
    let diagonal2 = [top_right, grid[i][j], bottom_left]; // Top-right to bottom-left

    // Match against MAS or SAM
    (diagonal1 == ['M', 'A', 'S'] || diagonal1 == ['S', 'A', 'M']) &&
        (diagonal2 == ['M', 'A', 'S'] || diagonal2 == ['S', 'A', 'M'])
}


fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines().map(|line| line.expect("Could not read line").chars().collect()).collect();


    let word = "XMAS";
    let part1_answer: usize = search_horizontal_and_vertical(&grid, word)
        + search_diagonals(&grid, word);

    let part2_answer: usize = search_xmas_patterns(&grid);

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

