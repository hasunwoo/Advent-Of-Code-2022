use std::{fs, str::FromStr};

use thiserror::Error;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let total_score = input
        .split('\n')
        .map(|s| {
            let mut s = s.split_whitespace();
            let opponent: Shape = s.next().unwrap().parse().unwrap();
            let outcome: Outcome = s.next().unwrap().parse().unwrap();
            compute_score(outcome, opponent)
        })
        .sum::<usize>();
    format!("{total_score}")
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Error, Debug)]
#[error("Error parsing shape for input: {0}")]
struct ParseShapeError(String);

impl FromStr for Shape {
    type Err = ParseShapeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = s
            .chars()
            .next()
            .ok_or_else(|| ParseShapeError(s.to_owned()))?;

        match char {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err(ParseShapeError(s.to_owned())),
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Error, Debug)]
#[error("Error parsing shape for input: {0}")]
struct ParseOutcomeError(String);

impl FromStr for Outcome {
    type Err = ParseOutcomeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = s
            .chars()
            .next()
            .ok_or_else(|| ParseOutcomeError(s.to_owned()))?;

        match char {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(ParseOutcomeError(s.to_owned())),
        }
    }
}

fn compute_score(outcome: Outcome, opponent: Shape) -> usize {
    let me = match opponent {
        Shape::Rock => match outcome {
            Outcome::Lose => Shape::Scissors,
            Outcome::Draw => Shape::Rock,
            Outcome::Win => Shape::Paper,
        },
        Shape::Paper => match outcome {
            Outcome::Lose => Shape::Rock,
            Outcome::Draw => Shape::Paper,
            Outcome::Win => Shape::Scissors,
        },
        Shape::Scissors => match outcome {
            Outcome::Lose => Shape::Paper,
            Outcome::Draw => Shape::Scissors,
            Outcome::Win => Shape::Rock,
        },
    };
    let shape_score = match me {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let outcome_score = match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };
    shape_score + outcome_score
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            solve(
                r#"A Y
B X
C Z"#
            ),
            "12"
        );
    }
}
