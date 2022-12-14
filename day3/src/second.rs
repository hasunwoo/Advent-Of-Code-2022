use std::hash::Hash;
use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let groups = input
        .lines()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .tuples::<(_, _, _)>();
    let common_items = groups
        .map(|(a, b, c)| *intersection(&[&a, &b, &c]).into_iter().next().unwrap())
        .map(|c| priority(c).unwrap())
        .sum::<usize>();
    format!("{common_items}")
}

fn intersection<'a, T: Hash + Eq>(sets: &[&'a HashSet<T>]) -> HashSet<&'a T> {
    if sets.is_empty() {
        return HashSet::new();
    }
    let mut result: HashSet<&T> = sets[0].iter().collect();
    result.retain(|e| sets.iter().all(|s| s.contains(e)));
    result
}

fn priority(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some(c as usize - 'a' as usize + 1),
        'A'..='Z' => Some(c as usize - 'A' as usize + 27),
        _ => None,
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
            "70"
        );
    }
}
