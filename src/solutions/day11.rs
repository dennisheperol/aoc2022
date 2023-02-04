use std::{fmt, fs};

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day11.txt").expect("Couldn't read file");

    let mut pack = parse_monkey_pack(&content);

    for _ in 0..20 {
        pack.do_round(|x| x / 3);
    }

    let solution = pack.get_monkey_business();

    println!("{solution}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day11.txt").expect("Couldn't read file");

    let mut pack = parse_monkey_pack(&content);
    let product = pack.product_of_monkey_tests();

    for _ in 0..10000 {
        pack.do_round(|x| x % product);
    }

    let solution = pack.get_monkey_business();

    println!("{solution}");
}

#[derive(Debug)]
struct MonkeyPack {
    monkeys: Vec<Monkey>,
}

impl MonkeyPack {
    fn do_round<F>(&mut self, worry_manager: F)
        where F: FnOnce(u64) -> u64 + Copy
    {
        for i in 0..self.monkeys.len() {
            while let Some((item, index)) = self.monkeys[i].throw_item(worry_manager) {
                self.monkeys[index].receive_item(item);
            }
        }
    }

    fn get_monkey_business(&self) -> usize {
        let mut inspections = self.monkeys
            .iter()
            .map(|monkey| monkey.inspections).collect::<Vec<usize>>();
        inspections.sort();
        inspections.reverse();
        inspections.iter().take(2).product()
    }

    fn product_of_monkey_tests(&self) -> u64 {
        self.monkeys.iter().map(|monkey| monkey.test).product()
    }
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u64,
    test_true: usize,
    test_false: usize,
    inspections: usize,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Monkey {{ items: {:?}, test: {:?}, test_true: {:?}, test_false {:?}, inspections: {:?} }}",
               self.items, self.test, self.test_true, self.test_false, self.inspections)
    }
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Box<dyn Fn(u64) -> u64>, test: u64, test_true: usize, test_false: usize) -> Monkey {
        Monkey { items, operation, test, test_true, test_false, inspections: 0 }
    }

    fn throw_item<F>(&mut self, worry_manager: F) -> Option<(u64, usize)>
        where F: FnOnce(u64) -> u64 + Copy
    {
        if self.items.len() == 0 {
            return None;
        }
        let mut item = self.items.remove(0);
        item = (self.operation)(item);
        item = worry_manager(item);

        let monkey = if item % self.test == 0 { self.test_true } else { self.test_false };
        self.inspections += 1;

        Some((item, monkey))
    }

    fn receive_item(&mut self, item: u64) {
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
    let test = &test[21..].parse::<u64>().unwrap();
    let test_true = &test_true[29..].parse::<usize>().unwrap();
    let test_false = &test_false[30..].parse::<usize>().unwrap();

    Monkey::new(items, operation, *test, *test_true, *test_false)
}

fn parse_operation(operation: &str) -> Box<dyn Fn(u64) -> u64> {
    if operation == "* old" {
        Box::new(|x| x.pow(2))
    } else if operation.starts_with("*") {
        let number = operation[2..].parse::<u64>().unwrap();
        Box::new(move |x| x * number)
    } else {
        let number = operation[2..].parse::<u64>().unwrap();
        Box::new(move |x| x + number)
    }
}