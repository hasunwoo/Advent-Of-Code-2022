#![feature(array_windows)]
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let marker_pos = identify_marker(input).unwrap();
    format!("{marker_pos}")
}

fn identify_marker(s: &str) -> Option<usize> {
    let chars = s.chars().collect::<Vec<_>>();
    let windows = chars.array_windows::<4>();
    windows
        .enumerate()
        .find(|(_, &w)| !has_duplicated_elements(&w))
        .map(|x| x.0 + 4)
}

fn has_duplicated_elements<T: PartialEq>(slice: &[T]) -> bool {
    for (i, e) in slice.iter().enumerate().take(slice.len() - 1) {
        if slice[i + 1..].contains(e) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }
}
