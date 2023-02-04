use std::fs;

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day10.txt").expect("Couldn't read file");

    let cycles = get_cycles(&content);

    let total = 20* cycles[19] + 60*cycles[59] + 100*cycles[99] + 140*cycles[139] + 180*cycles[179] + 220*cycles[219];

    println!("{total}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day10.txt").expect("Couldn't read file");

    let cycles = get_cycles(&content);

    draw_sprite(&cycles);
}

fn draw_sprite(positions : &Vec<i32>) {
    for row in 0..6 {
        for pixel in 0..40 {
            if in_range(pixel, positions[(pixel + row * 40) as usize]) {
                print!("#")
            } else { print!(".") }
        }
        println!();
    }
}

fn in_range(pixel: i32, sprite_middle: i32) -> bool {
    (sprite_middle - pixel).abs() <= 1
}

fn get_cycles(content: &str) -> Vec<i32> {
    let mut cycles = vec![1];

    for line in content.lines() {
        let last_one = *cycles.last().unwrap();
        if let Some((_, num)) = line.split_once(" ") {
            let num = num.parse::<i32>().unwrap();
            cycles.push(last_one);
            cycles.push(last_one + num);
        } else {
            cycles.push(last_one);
        }
    }

    cycles
}