use std::collections::HashSet;
use std::fs;

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day06.txt").expect("Couldn't read file");

    let i = find_distinct_chars(content, 14);

    println!("{i}");
}

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day06.txt").expect("Couldn't read file");

    let i = find_distinct_chars(content, 4);

    println!("{i}");
}

fn find_distinct_chars(content: String, size: usize) -> usize{
    let mut set: HashSet<char> = HashSet::new();
    for i in 0..content.len() - size {
        content[i..i + size]
            .chars()
            .for_each(|c| { set.insert(c); });
        if set.len() == size {
            return i + size;
        } else {
            set.clear();
        }
    }
    panic!("Couldn't find distinct chars")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_input() {}
}