use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day_2.txt").expect("Unable to parse input");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Copy, Clone)]
enum PolicyType {
    Occurrences,
    SingleMatch,
}

struct Policy {
    r#type: PolicyType,
    first_val: usize,
    second_val: usize,
    character: char,
}

#[derive(Debug)]
struct ParseError;

lazy_static! {
    static ref POLICY_RE: Regex = Regex::new("([0-9]+)-([0-9]+) ([a-z])").unwrap();
}

impl Policy {
    pub fn new(s: &str, policy_type: PolicyType) -> Result<Policy, ParseError> {
        let captures = POLICY_RE.captures(s).ok_or_else(|| ParseError {})?;

        let first_val: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let second_val: usize = captures.get(2).unwrap().as_str().parse().unwrap();
        let character: char = captures.get(3).unwrap().as_str().chars().next().unwrap();

        Ok(Policy {
            r#type: policy_type,
            first_val,
            second_val,
            character,
        })
    }

    pub fn valid(self: &Policy, password: &str) -> bool {
        match self.r#type {
            PolicyType::Occurrences => {
                let occurrences = password.chars().filter(|&c| c == self.character).count();
                occurrences >= self.first_val && occurrences <= self.second_val
            }
            PolicyType::SingleMatch => {
                let chars: Vec<char> = password.chars().collect();
                (chars[self.first_val - 1] == self.character)
                    ^ (chars[self.second_val - 1] == self.character)
            }
        }
    }
}

fn part_one(input: &str) -> usize {
    count_valid_passwords(input, PolicyType::Occurrences)
}

fn part_two(input: &str) -> usize {
    count_valid_passwords(input, PolicyType::SingleMatch)
}

fn count_valid_passwords(input: &str, policy_type: PolicyType) -> usize {
    let mut valid = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        let policy: Policy = Policy::new(parts[0], policy_type).expect("Unable to parse policy");
        let password = parts[1];

        if policy.valid(password) {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn test_part_one() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

        assert_eq!(part_one(input), 2)
    }

    #[test]
    fn test_part_two() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

        assert_eq!(part_two(input), 1)
    }
}
