use std::hash::Hash;
use std::{collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let groups = input
        .split('\n')
        .map(|s| s.chars().collect::<HashSet<char>>())
        .tuples::<(_, _, _)>();
    let common_items = groups
        .map(|(a, b, c)| intersection(&[&a, &b, &c]).into_iter().next().unwrap())
        .map(|c| priority(c).unwrap())
        .sum::<usize>();
    format!("{common_items}")
}

fn intersection<T: Clone + Hash + Eq>(sets: &[&HashSet<T>]) -> HashSet<T> {
    if sets.is_empty() {
        return HashSet::new();
    }
    if sets.len() == 1 {
        return sets[0].clone();
    }
    let mut result = sets[0].clone();
    result.retain(|e| sets.iter().all(|s| s.contains(e)));
    result
}

fn priority(c: char) -> Option<usize> {
    if ('a'..='z').contains(&c) {
        Some(c as usize - 'a' as usize + 1)
    } else if ('A'..='Z').contains(&c) {
        Some(c as usize - 'A' as usize + 27)
    } else {
        None
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
