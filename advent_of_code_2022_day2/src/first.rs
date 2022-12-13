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
            let me: Shape = s.next().unwrap().parse().unwrap();
            compute_score(opponent, me)
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
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(ParseShapeError(s.to_owned())),
        }
    }
}

fn compute_score(opponent: Shape, me: Shape) -> usize {
    let shape_score = match me {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let outcome_score = match opponent {
        Shape::Rock => match me {
            Shape::Rock => 3,
            Shape::Paper => 6,
            Shape::Scissors => 0,
        },
        Shape::Paper => match me {
            Shape::Rock => 0,
            Shape::Paper => 3,
            Shape::Scissors => 6,
        },
        Shape::Scissors => match me {
            Shape::Rock => 6,
            Shape::Paper => 0,
            Shape::Scissors => 3,
        },
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
            "15"
        );
    }
}
