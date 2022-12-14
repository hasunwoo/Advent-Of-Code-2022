use std::fs;

use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let count = input
        .lines()
        .map(|s| two_pairs(s).unwrap().1)
        .filter(|(a, b)| is_intersect(a, b) || is_intersect(b, a))
        .count();
    format!("{count}")
}

type Range = (usize, usize);

fn is_intersect(s1: &Range, s2: &Range) -> bool {
    !(s1.1 < s2.0 || s2.1 < s1.0)
}

fn num_u32(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn range(input: &str) -> IResult<&str, Range> {
    separated_pair(num_u32, char('-'), num_u32)(input)
}

fn two_pairs(input: &str) -> IResult<&str, (Range, Range)> {
    separated_pair(range, char(','), range)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            solve(
                r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
            ),
            "4"
        );
    }
}
