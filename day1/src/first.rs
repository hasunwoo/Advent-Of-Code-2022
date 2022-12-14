use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let calories: Vec<_> = input
        .split("\n\n")
        .map(|it| {
            it.split('\n')
                .map(|s| s.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();
    let max = calories.iter().max().unwrap();
    format!("{max}")
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
