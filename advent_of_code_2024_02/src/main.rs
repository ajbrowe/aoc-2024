use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe(levels: Vec<i32>) -> bool {

    let increasing: bool = levels[1] > levels[0];

    let mut last_level: i32 = -1;
    let mut count: i32 = 0;
    let mut is_safe: bool = true;
    
    //println!("First: {:?}, Second: {:?}, Increasing? {:?}", levels[0], levels[1], increasing);

    for level in levels {
        if count > 0 {
            let difference: i32 = (level - last_level).abs();
            if difference < 1 || difference > 3 {
                //println!("unsafe because difference is less than one or greater than 3: {:?} {:?}", level, last_level);
                is_safe = false;
            }
            else {
                //println!("difference abs({:?} - {:?}) = {:?}", level, last_level, difference);
            }
            if increasing && level < last_level {
                //println!("unsafe because increasing is now decreasing: {:?} {:?}", level, last_level);
                is_safe = false;
            }
            if !increasing && level > last_level {
                //println!("unsafe because decreasing is now increasing: {:?} {:?}", level, last_level);
                is_safe = false;
            }
        }

        last_level = level;
        count += 1;
    }

    return is_safe;


}

fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: i32 = 0;
    let mut part2_answer: i32 = 0;

    // Process each line
    for line in reader.lines() {
        let line = line?;
        let levels: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        if is_safe(levels.clone()) {
            part1_answer += 1;
            part2_answer += 1;
        }
        else {
            let mut n = 0;
            while n < levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(n);
                if is_safe(new_levels) {
                    part2_answer += 1;
                    break;
                }
                n += 1;
            }
        }
    }



    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

