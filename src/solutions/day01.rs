use std::fs;

pub fn part2() {
    solve(3);
}

pub fn part1() {
    solve(1);
}

pub fn solve(n_elves: usize) {
    let content = fs::read_to_string("puzzle_input/day01.txt").expect("Couldn't open file");

    let mut data = parse(&content);
    data.sort();
    data.reverse();

    let sum_n_elves: i32 = data.iter().take(n_elves).sum();

    println!("{}", sum_n_elves);
}

pub fn parse(content: &str) -> Vec<i32> {
    content
        .split("\n\n")
        .into_iter()
        .map(|s| s
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .reduce(|sum, num| sum + num)
            .unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let content = "\
1
2
3

5
6

8
";
        assert_eq!(vec![6, 11, 8], parse(content));
    }
}