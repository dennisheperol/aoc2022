use std::fs;

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day12.txt").expect("Couldn't read file");

    let mut map = parse_height_map(&content);

    let steps = map.steps_to_finish();

    println!("{steps}")
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day12.txt").expect("Couldn't read file");

    let mut map = parse_height_map(&content);
    map.override_start();
    let steps = map.steps_to_finish();

    println!("{steps}")
}

#[derive(Debug)]
struct Node {
    height: usize,
    is_finish: bool,
    is_visited: bool,
}

impl Node {
    fn new(height: usize, is_finish: bool) -> Node {
        Node { height, is_finish, is_visited: false }
    }
}

#[derive(Debug)]
struct HeightMap {
    nodes: Vec<Vec<Node>>,
    to_visit: Vec<(usize, usize)>,
}

impl HeightMap {
    fn from<'a>(nodes: Vec<Vec<Node>>, start: (usize, usize)) -> HeightMap {
        HeightMap { nodes, to_visit: vec![start] }
    }

    fn override_start(&mut self) {
        let mut next_to_visit = vec![];

        for row in 0..self.nodes.len() {
            for col in 0..self.nodes[row].len() {
                let node = &mut self.nodes[row][col];

                if node.height == 0 {
                    next_to_visit.push((row, col));
                    node.is_visited = true;
                }
            }
        }

        self.to_visit = next_to_visit;
    }

    fn steps_to_finish(&mut self) -> usize {
        let mut steps = 0;

        loop {
            if self.contains_finish() { break; }

            let mut next_to_visit = vec![];
            let nodes_to_visit = self.to_visit.clone();

            for next_node in nodes_to_visit {
                next_to_visit.append(&mut self.visit_nodes_from(next_node));
            }

            self.to_visit = next_to_visit;
            steps += 1
        }

        steps
    }

    fn contains_finish(&self) -> bool {
        self.to_visit.iter()
            .map(|(row, col)| self.nodes[*row][*col].is_finish)
            .any(|finish| finish)
    }

    fn visit_nodes_from(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut visit = vec![];
        let own_height = self.nodes[start.0][start.1].height;

        if start.0 > 0 {
            let node = &mut self.nodes[start.0 - 1][start.1];
            if !node.is_visited && node.height <= own_height + 1 {
                node.is_visited = true;
                visit.push((start.0 - 1, start.1))
            };
        }
        if let Some(node_row) = self.nodes.get_mut(start.0 + 1) {
            if let Some(node) = node_row.get_mut(start.1) {
                if !node.is_visited && node.height <= own_height + 1 {
                    node.is_visited = true;
                    visit.push((start.0 + 1, start.1));
                }
            }
        }
        if let Some(node_row) = self.nodes.get_mut(start.0) {
            if start.1 > 0 {
                let node = &mut node_row[start.1 - 1];
                if !node.is_visited && node.height <= own_height + 1 {
                    node.is_visited = true;
                    visit.push((start.0, start.1 - 1));
                }
            }
            if let Some(node) = node_row.get_mut(start.1 + 1) {
                if !node.is_visited && node.height <= own_height + 1 {
                    node.is_visited = true;
                    visit.push((start.0, start.1 + 1));
                }
            }
        }

        visit
    }
}

fn parse_height_map(content: &str) -> HeightMap {
    let mut nodes = vec![];
    let mut start = None;

    for (row, line) in content.lines().enumerate() {
        nodes.push(vec![]);

        for (col, c) in line.chars().enumerate() {
            let node = if c.is_lowercase() {
                Node::new(c as usize - 97, false)
            } else if c == 'S' {
                start = Some((row, col));
                Node::new(0, false)
            } else {
                Node::new(25, true)
            };
            nodes[row].push(node);
        }
    }

    if let Some(start) = start {
        HeightMap::from(nodes, start)
    } else {
        panic!()
    }
}