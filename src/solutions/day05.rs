use std::fs;

#[derive(PartialEq, Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day05.txt").expect("Couldn't read file");

    let (all_containers, instructions) = split_input(&content);
    let mut all_containers = parse_containers(all_containers);

    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    for instruction in instructions {
        all_containers = move_multiple_containers(all_containers, instruction);
    }

    println!("{}", top_containers(all_containers))
}

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day05.txt").expect("Couldn't read file");

    let (all_containers, instructions) = split_input(&content);
    let mut all_containers = parse_containers(all_containers);

    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    for instruction in instructions {
        all_containers = move_containers(all_containers, instruction);
    }

    println!("{}", top_containers(all_containers))
}

fn split_input(input: &str) -> (&str, &str) {
    let split: Vec<&str> = input.split("\n\n").collect();

    (split[0], split[1])
}

fn parse_containers(input: &str) -> Vec<Vec<char>> {
    let mut input_iter = input
        .lines()
        .rev();

    let number_row = input_iter.next().expect("Wrong input for containers");
    let amount_of_rows = (number_row.len() + 1) / 4;

    let mut container_rows = vec![Vec::new(); amount_of_rows];

    // fill vec in container_rows
    while let Some(container_line) = input_iter.next() {
        let container_chars: Vec<char> = container_line.chars().collect();
        for i in 0..amount_of_rows {
            match container_chars[i * 4 + 1] {
                ' ' => continue,
                c => container_rows[i].push(c),
            }
        }
    }

    container_rows
}

fn move_containers(mut containers: Vec<Vec<char>>, instruction: Instruction) -> Vec<Vec<char>> {
    for _ in 0..instruction.amount {
        let b = containers[instruction.from].pop().unwrap();
        containers[instruction.to].push(b);
    }
    containers
}

fn move_multiple_containers(mut containers: Vec<Vec<char>>, instruction: Instruction) -> Vec<Vec<char>> {
    let insert_at = containers[instruction.to].len();
    for _ in 0..instruction.amount {
        let b = containers[instruction.from].pop().unwrap();
        containers[instruction.to].insert(insert_at, b);
    }
    containers
}

fn top_containers(all_containers: Vec<Vec<char>>) -> String {
    let mut top = String::new();

    for containers in all_containers {
        if let Some(container) = containers.last() {
            top.push(*container);
        }
    }

    top
}

fn parse_instruction(input: &str) -> Instruction {
    let amount_idx = input.find("move ").expect("Couldn't find move");
    let from_idx = input.find(" from ").expect("Couldn't find from");
    let to_idx = input.find(" to ").expect("Couldn't find to");

    Instruction {
        amount: input[amount_idx + 5..from_idx].parse().expect("amount not a number"),
        from: input[from_idx + 6..to_idx].parse::<usize>().expect("from not a number") - 1,
        to: input[to_idx + 4..].parse::<usize>().expect("to not a number") - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_input() {
        let input = get_test_input();

        let split = split_input(&input);

        assert_eq!(split.0, "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ");
        assert_eq!(split.1, "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n");
    }

    #[test]
    fn test_parse_containers() {
        let input = get_test_input();
        let container_input = split_input(&input).0;

        assert_eq!(vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ], parse_containers(container_input))
    }

    #[test]
    fn test_move_containers() {
        let input = get_test_input();
        let container_input = split_input(&input).0;
        let mut containers = parse_containers(container_input);

        assert_eq!(vec![
            vec![],
            vec!['M', 'C', 'D'],
            vec!['P', 'N', 'Z'],
        ], move_containers(
            containers,
            Instruction {
                from: 0,
                to: 2,
                amount: 2,
            })
        )
    }

    #[test]
    fn test_move_multiple_containers() {
        let input = get_test_input();
        let container_input = split_input(&input).0;
        let mut containers = parse_containers(container_input);

        assert_eq!(vec![
            vec![],
            vec!['M', 'C', 'D'],
            vec!['P', 'Z', 'N'],
        ], move_multiple_containers(
            containers,
            Instruction {
                from: 0,
                to: 2,
                amount: 2,
            })
        )
    }

    #[test]
    fn test_top_containers() {
        let input = get_test_input();
        let container_input = split_input(&input).0;
        let containers = parse_containers(container_input);

        assert_eq!("NDP", top_containers(containers));
    }

    #[test]
    fn test_parse_instructions() {
        let input = get_test_input();
        let instruction_input = split_input(&input).1;

        assert_eq!(Instruction {
            amount: 1,
            from: 1,
            to: 0,
        }, parse_instruction(instruction_input.lines().next().unwrap()));
    }

    fn get_test_input() -> String {
        fs::read_to_string("puzzle_input/test/day05.txt").expect("Couldn't read file")
    }
}