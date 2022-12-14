extern crate core;

use std::collections::HashSet;
use std::fs;

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day03.txt").expect("Couldn't read file");

    let sum: i32 = content
        .lines()
        .map(|line| half_parts(line))
        .map(|parts| get_common_char(parts))
        .map(|c| get_char_value(c))
        .sum();

    println!("{sum}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day03.txt").expect("Couldn't read file");

    let mut lines = content.lines();
    let mut chars = Vec::new();

    'outer: loop {
        let mut three_lines = Vec::new();
        for _ in 0..3 {
            match lines.next() {
                Some(line) => three_lines.push(line),
                None => break 'outer
            }
        }
        chars.push(get_common_char(three_lines));
    }

    let sum: i32 = chars.iter()
        .map(|c| get_char_value(*c))
        .sum();

    println!("{}", sum);
}

fn half_parts(input: &str) -> Vec<&str> {
    let part1 = &input[..input.len() / 2];
    let part2 = &input[input.len() / 2..];
    vec![part1, part2]
}

fn get_common_char(parts: Vec<&str>) -> char {
    let common_chars: Vec<HashSet<char>> = parts
        .iter()
        .map(|part| HashSet::from_iter(part.chars()))
        .collect();

    let intersection = common_chars.iter()
        .fold(common_chars[0].clone(), |acc, set| {
            acc.intersection(&set).cloned().collect()
        });

    if intersection.len() == 1 {
        return intersection.iter().next().unwrap().clone();
    } else {
        panic!("Error in finding single common char")
    }
}

fn get_char_value(c: char) -> i32 {
    return if c.is_lowercase() {
        c as i32 - 96
    } else {
        c as i32 - 38
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_parts_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"], half_parts(input));
    }

    #[test]
    fn find_common_char_test() {
        let input = vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"];
        assert_eq!('p', get_common_char(input));
    }

    #[test]
    fn char_value_lowercase_test() {
        let c = 'b';
        assert_eq!(2, get_char_value(c));
    }

    #[test]
    fn char_value_uppercase_test() {
        let c = 'C';
        assert_eq!(29, get_char_value(c));
    }
}