use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day01_solver() -> io::Result<()> {
    let input = File::open("input/day01.txt")?;
    let reader = BufReader::new(input);

    let mut instructions: Vec<i32> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let (dir, value_str) = line.split_at(1);
        let mut value: i32 = value_str.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if dir == "L" {
            value *= -1;
        }
        instructions.push(value);
    }

    let part_one = count_zeros(&instructions);
    let part_two = pass_zeros(&instructions);

    println!("--- Day 1: Secret Entrance ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn count_zeros(instructions: &Vec<i32>) -> u32 {
    let mut zeros: u32 = 0;
    let mut starting_value: i32 = 50;

    for instruction in instructions {
        starting_value = ((starting_value + instruction % 100) + 100) % 100;
        if starting_value == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn pass_zeros(instructions: &Vec<i32>) -> u32 {
    let mut zeros: u32 = 0;
    let mut starting_value: i32 = 50;

    for instruction in instructions {
        let mut new_value: i32  = starting_value + instruction;

        if new_value == 0  {
            zeros += 1;
        }
        else if new_value >= 100 {
            while new_value >= 100 {
                zeros += 1;
                new_value -= 100;
            }
        }
        else if new_value < 0 {
            while new_value < 0 {
                zeros += 1;
                new_value += 100;
            }
            if starting_value == 0 {
                zeros -= 1;
            }
            if new_value == 0 {
                zeros += 1;
            }
        }

        starting_value = new_value;
    }
    zeros
}