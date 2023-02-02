mod solutions;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    // expect two arguments: day number, part number
    let day_num = argv[1].parse::<isize>().expect("arg 1 was not a number");
    let part = argv[2].parse::<isize>().expect("arg 2 was not a number");
    match (day_num, part) {
        (1, 1) => solutions::day01::part1(),
        (1, 2) => solutions::day01::part2(),
        (2, 1) => solutions::day02part1::solve(),
        (2, 2) => solutions::day02part2::solve(),
        (3, 1) => solutions::day03::part1(),
        (3, 2) => solutions::day03::part2(),
        (4, 1) => solutions::day04::part1(),
        (4, 2) => solutions::day04::part2(),
        (5, 1) => solutions::day05::part1(),
        (5, 2) => solutions::day05::part2(),
        (6, 1) => solutions::day06::part1(),
        (6, 2) => solutions::day06::part2(),
        (7, 1) => solutions::day07::part1(),
        (7, 2) => solutions::day07::part2(),
        (8, 1) => solutions::day08::part1(),
        (8, 2) => solutions::day08::part2(),
        (9, 1) => solutions::day09::part1(),
        (9, 2) => solutions::day09::part2(),
        _ => panic!("unknown day {}", day_num),
    };
}