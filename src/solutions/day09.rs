use std::collections::HashSet;
use std::fs;

use crate::solutions::day09::Direction::{DOWN, LEFT, RIGHT, UP};

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day09.txt").expect("Couldn't read file");

    let positions = get_tail_positions(&content, 2);

    println!("{positions}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day09.txt").expect("Couldn't read file");

    let positions = get_tail_positions(&content, 10);

    println!("{positions}");
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Rope {
    segments: Vec<(i32, i32)>,
    tail_pos: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        let mut segments = vec![];
        (0..size).for_each(|_| segments.push((0, 0)));
        let mut tail_pos = HashSet::new();
        tail_pos.insert((0, 0));

        Rope {
            segments,
            tail_pos
        }
    }

    fn do_move(&mut self, direction: &Direction, steps: usize) {
        for _ in 0..steps {
            self.move_head(direction);
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        let mut head = self.segments[0];
        head = match direction {
            UP => (head.0 + 1, head.1),
            DOWN => (head.0 - 1, head.1),
            LEFT => (head.0, head.1 - 1),
            RIGHT => (head.0, head.1 + 1),
        };
        self.segments[0] = head;

        for i in 1..self.segments.len() {
            self.check_move_segment(i);
        }

        self.save_tail_pos();
    }

    fn save_tail_pos(&mut self) {
        self.tail_pos.insert(*self.segments.last().unwrap());
    }

    fn check_move_segment(&mut self, segment: usize) {
        let head = self.segments[segment-1];
        let mut tail = self.segments[segment];

        if (head.0 - tail.0).abs() > 1 ||
            (head.1 - tail.1).abs() > 1 {
            let move_0 = (head.0 - tail.0).min(1).max(-1);
            let move_1 = (head.1 - tail.1).min(1).max(-1);

            tail = (tail.0 + move_0, tail.1 + move_1);
            self.segments[segment] = tail;
        }
    }

    fn get_unique_tail_positions(&self) -> usize {
        self.tail_pos.len()
    }
}

fn get_tail_positions(content: &String, rope_size: usize) -> usize {
    let mut rope = Rope::new(rope_size);

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