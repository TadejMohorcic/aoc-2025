use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::cmp::max;

pub fn day05_solver() -> io::Result<()> {
    let input = File::open("input/day05.txt")?;
    let reader = BufReader::new(input);

    let mut is_range = true;

    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;

        if line.is_empty() {
            is_range = false;
            continue;
        }
        
        if is_range {
            let range_str: Vec<&str> = line.split('-').collect();
            let low: u64 = range_str[0].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let high: u64 = range_str[1].parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            ranges.push((low, high));
        }
        else {
            let ingredient: u64 = line.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            ingredients.push(ingredient);
        }
    }

    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let joined_ranges = join_ranges(&ranges);

    let part_one = valid_ingredients(&joined_ranges, &ingredients);
    let part_two = total_valid_ingredients(&joined_ranges);

    println!("--- Day 5: Cafeteria ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn join_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut joined_ranges: Vec<(u64, u64)> = vec![];

    for range in ranges {
        let last_range = joined_ranges.pop();
        match last_range {
            Some((low, high)) => {
                if range.0 <= high {
                    joined_ranges.push((low, max(high, range.1)));
                }
                else {
                    joined_ranges.push((low, high));
                    joined_ranges.push(*range);
                }
            },
            None => joined_ranges.push(*range)
        }
    }

    joined_ranges
}

fn valid_ingredients(ranges: &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> u64 {
    let mut valid_ingredients: u64 = 0;

    for ingredient in ingredients {
        for range in ranges {
            if range.0 <= *ingredient && *ingredient <= range.1 {
                valid_ingredients += 1;
                break;
            }
        }
    }

    valid_ingredients
}

fn total_valid_ingredients(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut valid_ingredients: u64 = 0;

    for range in ranges {
        valid_ingredients += range.1 - range.0 + 1;
    }

    valid_ingredients
}