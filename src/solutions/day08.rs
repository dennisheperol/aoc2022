use std::fs;

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day08.txt").expect("Couldn't read file");

    let mut trees = parse_trees(content);
    let count = trees.count_visible_trees();

    println!("{count}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day08.txt").expect("Couldn't read file");

    println!("{content}");
}

#[derive(Debug)]
struct Tree {
    height: i8,
    seen: bool,
}

impl Tree {
    fn new(height: i8) -> Tree {
        Tree {
            height,
            seen: false,
        }
    }
}

#[derive(Debug)]
struct Trees {
    trees: Vec<Vec<Tree>>,
}

impl Trees {
    fn new() -> Trees {
        Trees {
            trees: vec![],
        }
    }

    fn add_tree(&mut self, height: i8, col: usize, row: usize) {
        if col == 0 {
            self.trees.push(vec![]);
        }
        self.trees[row].push(Tree::new(height));
    }

    fn count_visible_trees(&mut self) -> usize {
        self.set_trees_visible();

        let mut count = 0;
        for tree_row in &self.trees {
            for tree in tree_row {
                if tree.seen { count += 1 }
            }
        }
        count
    }

    fn set_trees_visible(&mut self) {
        for tree_row in &mut self.trees {
            let mut highest_tree = -1;
            for tree in tree_row {
                if tree.height > highest_tree {
                    highest_tree = tree.height;
                    tree.seen = true;
                    if highest_tree == 9 {
                        break;
                    }
                }
            }
        }

        for tree_row in &mut self.trees {
            let mut highest_tree = -1;
            for tree in tree_row.into_iter().rev() {
                if tree.height > highest_tree {
                    highest_tree = tree.height;
                    tree.seen = true;
                    if highest_tree == 9 {
                        break;
                    }
                }
            }
        }

        for col in 0..self.trees[0].len() {
            let mut highest_tree = -1;
            for row in 0..self.trees.len() {
                let tree = &mut self.trees[row][col];
                if tree.height > highest_tree {
                    highest_tree = tree.height;
                    tree.seen = true;
                    if highest_tree == 9 {
                        break;
                    }
                }
            }
        }

        for col in 0..self.trees[0].len() {
            let mut highest_tree = -1;
            for row in (0..self.trees.len()).rev() {
                let tree = &mut self.trees[row][col];
                if tree.height > highest_tree {
                    highest_tree = tree.height;
                    tree.seen = true;
                    if highest_tree == 9 {
                        break;
                    }
                }
            }
        }
    }
}

fn parse_trees(content: String) -> Trees {
    let mut trees = Trees::new();

    for (row, line) in content.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '\n' { continue; }
            let num = i8::try_from(char.to_digit(10).unwrap()).unwrap();
            trees.add_tree(num, col, row);
        }
    }
    trees
}