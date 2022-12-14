use std::{collections::HashSet, convert::Infallible, fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let priority_sum = input
        .lines()
        .map(|s| {
            let sack = s.parse::<Rucksack>().unwrap();
            let shared = sack.get_shared_items();
            shared.map(|&c| priority(c).unwrap()).sum::<usize>()
        })
        .sum::<usize>();
    format!("{priority_sum}")
}

fn priority(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some(c as usize - 'a' as usize + 1),
        'A'..='Z' => Some(c as usize - 'A' as usize + 27),
        _ => None,
    }
}

#[derive(Default)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Rucksack {
    fn get_shared_items(&self) -> impl Iterator<Item = &char> {
        self.first.intersection(&self.second)
    }
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Default::default());
        }
        let (first, second) = s.split_at(s.len() / 2);
        let first = first.chars().collect();
        let second = second.chars().collect();
        Ok(Rucksack { first, second })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            solve(
                r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
            ),
            "157"
        );
    }
}
