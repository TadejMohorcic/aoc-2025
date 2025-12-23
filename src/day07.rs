use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day07_solver() -> io::Result<()> {
    let input = File::open("input/day07.txt")?;
    let reader = BufReader::new(input);

    let mut start_pos: usize = 0;
    let mut splitter_locations: Vec<Vec<usize>> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let char_list: Vec<char> = line.chars().collect();
        let mut row_splitters: Vec<usize> = vec![];

        for (i, char) in char_list.iter().enumerate() {
            if *char == '^' {
                row_splitters.push(i);
            }
            else if *char == 'S' {
                start_pos = i;
            }
        }

        if !row_splitters.is_empty() {
            splitter_locations.push(row_splitters)
        }
    }
    let (part_one, part_two) = move_in_manifold(&splitter_locations, start_pos);

    println!("--- Day 7: Laboratories ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn move_in_manifold(splitters: &Vec<Vec<usize>>, start_pos: usize) -> (u64, u64) {
    let mut number_of_splits: u64 = 0;
    let mut active_beams: [u64; 141] = [0; 141];
    active_beams[start_pos] += 1;

    for row in splitters {
        for pos in row {
            let value = active_beams[*pos];
            
            if value != 0 {
                active_beams[*pos] = 0;
                active_beams[*pos + 1] += value;
                active_beams[*pos - 1] += value;
                number_of_splits += 1;
            }
        }
    }

    let timelines: u64 = active_beams.iter().fold(0, |acc, x| acc + x);

    (number_of_splits, timelines)
}