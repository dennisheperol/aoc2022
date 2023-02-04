use std::{fmt, fs};

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day11.txt").expect("Couldn't read file");

    let pack = parse_monkey_pack(&content);

    println!("{pack:#?}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day11.txt").expect("Couldn't read file");

    println!("{content}");
}

#[derive(Debug)]
struct MonkeyPack {
    monkeys: Vec<Monkey>,
}

impl MonkeyPack {
    fn do_round(&mut self) {
        for monkey in &mut self.monkeys {
            while let Some((item, index)) = monkey.throw_item() {
                self.monkeys[index].receive_item(item);
            }
        }
    }
}

struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: i32,
    test_true: usize,
    test_false: usize,
    inspections: usize
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey {{ items: {:?}, test: {:?}, test_true: {:?}, test_false {:?} }}", self.items, self.test, self.test_true, self.test_false)
    }
}

impl Monkey {
    fn new(items: Vec<i32>, operation: Box<dyn Fn(i32) -> i32>, test: i32, test_true: usize, test_false: usize) -> Monkey {
        Monkey { items, operation, test, test_true, test_false, inspections: 0 }
    }

    fn throw_item(&mut self) -> Option<(i32, usize)>{
        if self.items.len() == 0 {
            return None;
        }
        let item = self.items.remove(0);
        let item = (self.operation)(item) / 3;
        let monkey = if item % self.test == 0 { self.test_true } else { self.test_false };
        self.inspections += 1;

        Some((item, monkey))
    }

    fn receive_item(&mut self, item: i32) {
        self.items.push(item);
    }
}

fn parse_monkey_pack(content: &str) -> MonkeyPack {
    let monkeys = content.split("\n\n")
        .map(|monkey| parse_monkey(monkey))
        .collect();

    MonkeyPack { monkeys }
}

fn parse_monkey(monkey_string: &str) -> Monkey {
    let mut iter = monkey_string.lines().skip(1).into_iter();

    let (items, operation, test, test_true, test_false)
        = (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap());

    let items = items[18..].split(", ").map(|s| s.parse().unwrap()).collect();
    let operation = parse_operation(&operation[23..]);
    let test = &test[21..].parse::<i32>().unwrap();
    let test_true = &test_true[29..].parse::<usize>().unwrap();
    let test_false = &test_false[30..].parse::<usize>().unwrap();

    Monkey::new(items, operation, *test, *test_true, *test_false)
}

fn parse_operation(operation: &str) -> Box<dyn Fn(i32) -> i32> {
    if operation == "* old" {
        Box::new(|x| x^2)
    } else if operation.starts_with("*") {
        let number = operation[2..].parse::<i32>().unwrap();
        Box::new(move |x| x * number)
    } else {
        let number = operation[2..].parse::<i32>().unwrap();
        Box::new(move |x| x + number)
    }
}