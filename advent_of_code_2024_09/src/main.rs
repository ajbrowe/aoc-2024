use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DiskSpace {
    id: usize,
    length: usize,
    file: bool,
}


fn find_tail_file_ds (diskmap: &Vec<DiskSpace>, start: usize, last: usize) -> (usize, Option<DiskSpace>) {
    let mut search_idx: usize = last;
    if start >= last || last >= diskmap.len() {
        return (last, None);
    }
    while search_idx > start && (!diskmap[search_idx].file || diskmap[search_idx].length == 0) {
        search_idx -= 1;
    }
    let found = diskmap[search_idx];
    if search_idx > start && found.file && found.length > 0 {
        return (search_idx, Some(found));
    }
    return (search_idx, None);
}

fn find_first_space_idx (diskmap: &Vec<DiskSpace>, size: usize, limit: usize) -> Option<usize> {
    for (idx, ds) in diskmap.iter().enumerate() {
        if !ds.file && ds.length >= size && idx < limit {
            return Some(idx);
        }
    }
    None
}

fn print_diskmap (diskmap: &Vec<DiskSpace>) {

    for ds in diskmap {
        for _ in 0..ds.length {
            if ds.file {
                print!("{}", ds.id);
            }
            else {
                print!(".");
            }
        }
    }        
    println!("");


}

fn find_position<T: PartialEq>(vec: &Vec<T>, value: T) -> Option<usize> {
    vec.iter().position(|x| *x == value)
}


fn main() -> io::Result<()> {


    // Open the file
    let path = Path::new("input");
    //let path = Path::new("sample");
    //let path = Path::new("sample2");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut part1_answer: usize = 0;
    let mut part2_answer: usize = 0;


    let mut diskmap: Vec<DiskSpace> = Vec::new();

    // Process each line
    for line in reader.lines() {
        let line = line?;
        for (col_idx, ch) in line.chars().enumerate() {
            let is_file: bool = col_idx % 2 == 0;
            let file_id: usize = col_idx / 2;
            let length = ch.to_digit(10).unwrap_or(0) as usize;
            // only store files or non-zero space
            if is_file || length > 0 {
                diskmap.push(DiskSpace {
                    id: file_id,
                    length: length,
                    file: is_file,
                });
            }
        }
    }



    let mut last_tail_file_idx = diskmap.len() - 1;
    let mut contiguous_diskmap: Vec<DiskSpace> = Vec::new();

    let mut remaining_diskspace: HashMap<DiskSpace,usize> = HashMap::new();
    let mut moved_diskspace: HashMap<DiskSpace,usize> = HashMap::new();

    for (idx, ds) in diskmap.iter().enumerate() {
        if last_tail_file_idx <= idx {
            break;
        }
        if ds.file {
            //println!("inserting file block {:?}", ds);
            contiguous_diskmap.push(ds.clone());
        }
        else {
            let mut space = ds.length;
            while space > 0 {
                if let (tail_idx, Some(tail_ds)) = find_tail_file_ds(&diskmap, idx, last_tail_file_idx) {
                    last_tail_file_idx = tail_idx;
                    let mut tail_length = *remaining_diskspace.entry(tail_ds).or_insert(tail_ds.length);
                    let new_ds = DiskSpace{
                        id: tail_ds.id,
                        length: if space < tail_length { space } else { tail_length },
                        file: true,
                    };
                    //println!("creating new file block {:?} from {:?}", new_ds, tail_ds);
                    contiguous_diskmap.push(new_ds);
                    if space >= tail_length {
                        last_tail_file_idx -= 1;
                    }
                    tail_length -= new_ds.length;
                    if tail_length > 0 {
                        *remaining_diskspace.entry(tail_ds).or_insert(0) = tail_length;
                    }
                    else {
                        remaining_diskspace.remove(&tail_ds);
                    }
                    //tail_ds.length -= new_ds.length;
                    space -= new_ds.length;
                    //println!("tail file block now {:?} new length {}", tail_ds, tail_length);
                    //println!("remaining space to fill: {}", space);
                }
                else {
                    break;
                }
            }
        }
    }
    //println!("remaining {:?}", remaining_diskspace);
    for (ds, remaining_length) in remaining_diskspace {
        if remaining_length > 0 {
            contiguous_diskmap.push(DiskSpace { id: ds.id, length: remaining_length, file: true } );
        }
    }


    let mut block_counter: usize = 0;
    for ds in contiguous_diskmap {
        for _ in 0..ds.length {
            part1_answer += block_counter * ds.id;
            block_counter += 1;
            //print!("{}", ds.id);
        }
    }

    let mut defragged = diskmap.to_vec();
    let mut insert_count: usize = 0;
    //print_diskmap(&defragged);
    for idx in (0..diskmap.len()).rev() {
        let ds = diskmap[idx];
        if !ds.file || moved_diskspace.contains_key(&ds) {
            continue;
        }
        let original_pos = find_position(&defragged, ds).unwrap_or(idx + insert_count);
        if let Some(new_idx) = find_first_space_idx(&defragged, ds.length, original_pos) {
            let target = defragged[new_idx];
            //println!("Moving {:?} to position {} from {}", ds, new_idx, original_pos);
            let difference = target.length - ds.length;

            defragged.swap(new_idx, original_pos);
            if difference > 0 {
                defragged[original_pos].length = ds.length;
                defragged.insert(new_idx + 1, DiskSpace{ id: 0, length: target.length - ds.length, file: false});
                insert_count += 1;
            }
            //print_diskmap(&defragged);
            moved_diskspace.insert(ds, 1);
        }
    }


    //print_diskmap(&defragged);
    let mut block_counter: usize = 0;
    for ds in defragged {
        for _ in 0..ds.length {
            if ds.file {
                part2_answer += block_counter * ds.id;
            }
            block_counter += 1;
        }
    }        

    println!("Part1: {:?}", part1_answer);
    println!("Part2: {:?}", part2_answer);

    Ok(())
}

