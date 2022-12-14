use std::{fs, cmp::Reverse};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let calories = {
        let mut tmp: Vec<_> = input
            .split("\n\n")
            .map(|it| {
                it.split('\n')
                    .map(|s| s.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect();
        tmp.sort_unstable_by_key(|k| Reverse(*k));
        tmp
    };

    let top_three_sum = calories[0] + calories[1] + calories[2];

    format!("{top_three_sum}")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            solve(
                r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#
            ),
            "24000"
        );
    }
}
