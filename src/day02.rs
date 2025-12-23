use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::collections::HashSet;

pub fn day02_solver() -> io::Result<()> {
    let input = File::open("input/day02.txt")?;
    let reader = BufReader::new(input);
    
    let mut ranges: Vec<(u64, u64)> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let line_split: Vec<&str> = line.split(',').collect();

        for l in line_split {
            let test: Vec<&str> = l.split('-').collect();
            let low: u64 = test[0].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let high: u64 = test[1].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            ranges.push((low, high));
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mask_1: Vec<Vec<u64>> = [[11].to_vec(), [101].to_vec(), [1001].to_vec(), [10001].to_vec(), [100001].to_vec()].to_vec();
    let mask_2: Vec<Vec<u64>> = [[11, 111, 1111, 11111, 111111, 1111111, 11111111, 111111111, 1111111111].to_vec(),
    [101, 10101, 1010101, 101010101].to_vec(), [1001, 1001001].to_vec(), [10001].to_vec(), [100001].to_vec()].to_vec();

    let part_one = find_valid_ids(&ranges, &mask_1);
    let part_two = find_valid_ids(&ranges, &mask_2);

    println!("--- Day 2: Gift Shop ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn find_valid_ids(ranges: &Vec<(u64, u64)>, mask: &Vec<Vec<u64>>) -> u64 {
    let mut valid_ids: HashSet<u64> = HashSet::new();

    for i in 1..=99999_u64 {
        let number_of_digits = i.checked_ilog10().unwrap_or(0) + 1;
        let generator_id = number_of_digits as usize - 1;

        for generator in &mask[generator_id] {
            let generated_number = i * generator;

            for range in ranges {
                if range.0 <= generated_number && generated_number <= range.1 {
                    valid_ids.insert(generated_number);
                    break;
                }
            }
        }
    }

    let valids: Vec<u64> = valid_ids.into_iter().collect();
    let sum_of_valid_ids: u64 = valids.iter().fold(0, |acc, x| acc + x);

    sum_of_valid_ids
}

