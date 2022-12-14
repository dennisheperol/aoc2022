use std::fs;

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day04.txt").expect("Couldn't read file");
}

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day04.txt").expect("Couldn't read file");

    let sum = sum_fully_overlapping_sectors(&content);

    println!("{sum}");
}

fn sum_fully_overlapping_sectors(all_sectors: &str) -> i32 {
    all_sectors
        .lines()
        .map(|line| parse_sectors(line))
        .map(|sec| sectors_overlap(sec.0, sec.1))
        .map(|overlaps| if overlaps { 1 } else { 0 })
        .sum()
}

fn parse_sectors(sectors: &str) -> ((u32, u32), (u32, u32)) {
    let split: Vec<u32> =
        sectors.split(",")
            .flat_map(|sec| sec.split("-").collect::<Vec<&str>>())
            .map(|sec| sec.parse::<u32>().expect("Not a number"))
            .collect();

    ((split[0], split[1]), (split[2], split[3]))
}

fn sectors_overlap(sector1: (u32, u32), sector2: (u32, u32)) -> bool {
    (sector1.0 <= sector2.0 && sector1.1 >= sector2.1)
        || (sector1.0 >= sector2.0 && sector1.1 <= sector2.1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sectors_overlap_no_overlap() {
        assert!(!sectors_overlap((2, 4), (6, 8)));
    }

    #[test]
    fn test_sectors_overlap_partial_overlap() {
        assert!(!sectors_overlap((2, 6), (4, 8)));
    }

    #[test]
    fn test_sectors_overlap_full_overlap() {
        assert!(sectors_overlap((2, 6), (3, 5)));
    }

    #[test]
    fn test_sectors_overlap_full_overlap_reverse() {
        assert!(sectors_overlap((4, 5), (3, 6)));
    }

    #[test]
    fn test_sectors_overlap_same_sectors() {
        assert!(sectors_overlap((1, 10), (1, 10)));
    }

    #[test]
    fn test_sectors_overlap_same_sectors_single() {
        assert!(sectors_overlap((5, 5), (5, 5)));
    }

    #[test]
    fn test_parse_sectors() {
        let actual = parse_sectors("2-5,15-90");
        assert_eq!(((2, 5), (15, 90)), actual);
    }

    #[test]
    fn test_sum_overlapping_sectors() {
        let all_sectors = "2-5,15-90\n5-5,4-6\n3-8,6-8\n";
        assert_eq!(2, sum_fully_overlapping_sectors(all_sectors));
    }
}