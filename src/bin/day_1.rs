use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("data/day_1.txt").unwrap();
    println!("Part one: {:?}", part_one(&input));
    println!("Part two: {:?}", part_two(&input));
}

fn part_one(input: &str) -> Option<i32> {
    let mut values: HashSet<i32> = HashSet::new();

    for line in input.lines() {
        let value: i32 = line.parse::<_>().unwrap();

        if values.contains(&(2020 - value)) {
            return Some(value * (2020 - value));
        }

        values.insert(value);
    }

    None
}

fn part_two(input: &str) -> Option<i32> {
    let values: Vec<i32> = input.lines().map(|l| l.parse::<_>().unwrap()).collect();

    for (first_idx, first) in values.iter().enumerate() {
        for (second_idx, second) in values.iter().skip(first_idx + 1).enumerate() {
            for third in values.iter().skip(second_idx + 1) {
                if first + second + third == 2020 {
                    return Some(first * second * third);
                }
            }
        }
    }

    None
}
