use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn day03_solver() -> io::Result<()> {
    let input = File::open("input/day03.txt")?;
    let reader = BufReader::new(input);

    let mut batteries: Vec<Vec<u64>> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let digits_u64: Vec<u64> = digits.iter().map(|&x| x as u64).collect();
        batteries.push(digits_u64);
    }

    let part_one = calculate_joltage(&batteries, 2);
    let part_two = calculate_joltage(&batteries, 12);

    println!("--- Day 3: Lobby ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn calculate_joltage(battery_shelf: &Vec<Vec<u64>>, size: usize) -> u64 {
    let mut voltage: u64 = 0;

    for battery_bank in battery_shelf {
        let mut acc: Vec<&u64> = vec![];
        let n: usize = battery_bank.len();

        for (i, battery) in battery_bank.iter().enumerate() {
            let remaining: usize = n - 1 - i;

            while !acc.is_empty() && acc[acc.len() - 1] < battery && remaining >= size - acc.len() {
                acc.pop();
            }

            if acc.len() < size {
                acc.push(battery);
            }
        }

        let current_voltage = acc.iter().fold(0, |acc, x| 10 * acc + *x);
        voltage += current_voltage;
    }

    voltage
}