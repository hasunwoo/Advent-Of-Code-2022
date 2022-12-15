#![feature(get_many_mut)]
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, line_ending, not_line_ending, space0},
    combinator::{eof, map, map_res, recognize},
    multi::{count, many0, separated_list0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let (input, stacks) = stacks(input).unwrap();
    let (_input, commands) = move_commands(input).unwrap();

    let mut stacks = to_stack_list(&stacks);
    for cmd in commands {
        let from = cmd.from - 1;
        let to = cmd.to - 1;
        assert_ne!(from, to);
        assert!(from < stacks.len() && to <= stacks.len());
        let [from_vec, to_vec] = stacks.get_many_mut([from, to]).unwrap();
        let moves = from_vec.drain(from_vec.len() - cmd.amount..);
        to_vec.extend(moves);
    }

    let top_crates = stacks.iter().flat_map(|s| s.last()).collect::<String>();
    top_crates
}

fn slot(input: &str) -> IResult<&str, Option<char>> {
    alt((
        map(tag("   "), |_| None),
        map(delimited(char('['), anychar, char(']')), Some),
    ))(input)
}

fn row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list0(char(' '), slot), line_ending.or(eof))(input)
}

fn stacks(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    let (input, rows) = many0(row)(input)?;
    //consume two lines(number tags, empty line)
    let (input, _) = count(recognize(terminated(not_line_ending, line_ending)), 2)(input)?;
    Ok((input, rows))
}

fn to_stack_list(stack: &Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    assert!(!stack.is_empty());
    let mut stack_list = vec![vec![]; stack[0].len()];
    for row in stack.iter().rev() {
        for (c, col) in row.iter().enumerate() {
            if let Some(col) = col {
                stack_list[c].push(*col);
            }
        }
    }
    stack_list
}

#[derive(Debug)]
struct MoveCommand {
    amount: usize,
    from: usize,
    to: usize,
}

fn move_command(input: &str) -> IResult<&str, MoveCommand> {
    let number = || {
        delimited(
            space0,
            map_res(digit1, |s: &str| s.parse::<usize>()),
            space0,
        )
    };
    let (input, (amount, from, to)) = terminated(
        tuple((
            preceded(tag("move"), number()),
            preceded(tag("from"), number()),
            preceded(tag("to"), number()),
        )),
        line_ending.or(eof),
    )(input)?;
    Ok((input, MoveCommand { amount, from, to }))
}

fn move_commands(input: &str) -> IResult<&str, Vec<MoveCommand>> {
    many0(move_command)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            solve(
                r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
            ),
            "MCD"
        );
    }
}
