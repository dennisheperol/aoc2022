use std::fs;

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day08.txt").expect("Couldn't read file");

    let mut trees = parse_trees(content);
    let count = trees.count_visible_trees();

    println!("{count}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day08.txt").expect("Couldn't read file");

    let trees = parse_trees(content);
    let largest = trees.get_largest_scenic_score();

    println!("{largest}");
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

    fn get_largest_scenic_score(&self) -> usize {
        let mut largest = 0;

        for row in 1..(self.trees.len()-1) {
            for col in 1..(self.trees[row].len()-1){
                let scenic_score = self.get_scenic_score(row, col);
                if scenic_score > largest {
                    largest = scenic_score;
                }
            }
        }

        largest
    }

    fn get_scenic_score(&self, tree_row: usize, tree_col: usize) -> usize {

        let tree = &self.trees[tree_row][tree_col];

        let mut left = 0;
        for comp_col in (0..tree_col).rev() {
            left += 1;
            let comp_tree = &self.trees[tree_row][comp_col];
            if comp_tree.height >= tree.height { break; }
        }
        let mut right = 0;
        for comp_col in (tree_col + 1)..self.trees[tree_row].len() {
            right += 1;
            let comp_tree = &self.trees[tree_row][comp_col];
            if comp_tree.height >= tree.height { break; }
        }

        let mut up = 0;
        for comp_row in (0..tree_row).rev() {
            up += 1;
            let comp_tree = &self.trees[comp_row][tree_col];
            if comp_tree.height >= tree.height { break; }
        }
        let mut down = 0;
        for comp_row in (tree_row + 1)..self.trees.len() {
            down += 1;
            let comp_tree = &self.trees[comp_row][tree_col];
            if comp_tree.height >= tree.height { break; }
        }

        left * right * up * down
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