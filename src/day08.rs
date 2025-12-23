use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::collections::HashSet;

pub fn day08_solver() -> io::Result<()> {
    let input = File::open("input/day08.txt")?;
    let reader = BufReader::new(input);

    let mut junction_boxes: Vec<[i64; 3]> = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        let coordinates_vec: Vec<i64> = line.split(',').filter_map(|x| x.parse::<i64>().ok()).collect();
        let coordinates: [i64; 3] = coordinates_vec.try_into().unwrap();
        junction_boxes.push(coordinates);
    }
    
    let (part_one, part_two) = connect_junctions(&junction_boxes, 1000);

    println!("--- Day 8: Playground ---");
    println!(" - Part one solution: {}", part_one);
    println!(" - Part two solution: {}", part_two);
    println!("");

    Ok(())
}

fn calculate_distance(p1: [i64; 3], p2: [i64; 3]) -> i64 {
    let mut distance: i64 = 0;

    for i in 0..3 {
        distance += (p1[i] - p2[i]).pow(2);
    }

    distance
}

fn calculate_distances(junctions: &Vec<[i64; 3]>) -> Vec<(i64, usize, usize)> {
    let n: usize = junctions.len();
    let mut distances: Vec<(i64, usize, usize)> = vec![];

    for i in 0..n {
        for j in i+1..n {
            let distance = calculate_distance(junctions[i], junctions[j]);
            distances.push((distance, i, j));
        }
    }

    distances.sort_by_key(|tuple| tuple.0);
    distances
}

fn connect_junctions(junction_boxes: &Vec<[i64; 3]>, steps: usize) -> (usize, i64) {
    let mut part_one: usize = 1;
    let mut part_two: i64 = 0;

    let n = junction_boxes.len();
    let mut groups: Vec<Vec<usize>> = vec![];

    for i in 0..n {
        groups.push([i].to_vec());
    }

    let distances = calculate_distances(junction_boxes);

    for i in 0..usize::MAX {
        let p1 = distances[i].1;
        let p2 = distances[i].2;

        let mut first_group = groups[p1].clone();
        let mut second_group = groups[p2].clone();

        if first_group.contains(&p2) {
            continue;
        }

        first_group.append(&mut second_group);
        let mut seen = HashSet::new();
        first_group.retain(|x| seen.insert(*x));

        for i in &first_group {
            groups[*i] = first_group.clone();
        }

        if i == steps - 1 {
            let mut group_size: Vec<usize> = groups.iter().map(|x| x.len()).collect();
            group_size.sort_by(|a, b| b.cmp(&a));

            let mut index = 0;
            for _ in 0..3 {
                let val = group_size[index];
                part_one *= val;
                index += val;
            }
        }

        if first_group.len() == n {
            part_two = junction_boxes[distances[i].1][0] * junction_boxes[distances[i].2][0];
            break;
        }
    }

    (part_one, part_two)
}