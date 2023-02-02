use std::collections::HashSet;
use std::fs;

use crate::solutions::day09::Direction::{DOWN, LEFT, RIGHT, UP};

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day09.txt").expect("Couldn't read file");

    let positions = get_tail_positions(&content);

    println!("{positions}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day09.txt").expect("Couldn't read file");

    println!("{content}");
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    tail_pos: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Rope {
        let mut tail_pos = HashSet::new();
        tail_pos.insert((0, 0));

        Rope {
            head: (0, 0),
            tail: (0, 0),
            tail_pos,
        }
    }

    fn do_move(&mut self, direction: &Direction, steps: usize) {
        for _ in 0..steps {
            self.move_head(direction);
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        self.head = match direction {
            UP => (self.head.0 + 1, self.head.1),
            DOWN => (self.head.0 - 1, self.head.1),
            LEFT => (self.head.0, self.head.1 - 1),
            RIGHT => (self.head.0, self.head.1 + 1),
        };
        self.check_move_tail();
    }

    fn check_move_tail(&mut self) {
        let head = self.head;
        let mut tail = self.tail;

        if (head.0 - tail.0).abs() > 1 ||
            (head.1 - tail.1).abs() > 1 {
            let move_0 = (head.0 - tail.0).min(1).max(-1);
            let move_1 = (head.1 - tail.1).min(1).max(-1);

            tail = (tail.0 + move_0, tail.1 + move_1);
            self.tail = tail;
            self.tail_pos.insert(tail);
        }
    }

    fn get_unique_tail_positions(&self) -> usize {
        self.tail_pos.len()
    }
}

fn get_tail_positions(content: &String) -> usize {
    let mut rope = Rope::new();

    for line in content.lines() {
        let movement = parse_movement(line);
        rope.do_move(&movement.0, movement.1);
    };

    rope.get_unique_tail_positions()
}

fn parse_movement(movement: &str) -> (Direction, usize) {
    let (movement, amount) = movement.split_once(" ").unwrap();

    let direction = match movement {
        "U" => UP,
        "D" => DOWN,
        "L" => LEFT,
        "R" => RIGHT,
        _ => panic!()
    };
    let steps = amount.parse::<usize>().unwrap();

    (direction, steps)
}