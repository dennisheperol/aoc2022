use std::cell::RefCell;
use std::fs;
use std::rc::{Rc, Weak};

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day07.txt").expect("Couldn't read file");

    let size = build_filesystem(content).get_sizes_under(100000);

    println!("{size}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day07.txt").expect("Couldn't read file");

    println!("{content}");
}

#[derive(Debug)]
struct Directory {
    name: String,
    size: Rc<RefCell<usize>>,
    parent: RefCell<Weak<Directory>>,
    children: RefCell<Vec<Rc<Directory>>>,
}

impl Directory {
    fn new() -> Rc<Directory> {
        Rc::new(Directory {
            name: String::from("/"),
            size: Rc::new(RefCell::new(0)),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![])
        })
    }

    fn open_dir(self: &Rc<Directory>, name: String) -> Option<Rc<Directory>> {
        if let Some(dir) = self.children.borrow().iter()
            .find(|&dir| dir.name == name) {
            return Some(Rc::clone(dir));
        }
        None
    }

    fn create_dir(self: &Rc<Directory>, name: String) -> Rc<Directory> {
        let dir = Rc::new(Directory {
            name,
            size: Rc::new(RefCell::new(0)),
            parent: RefCell::new(Rc::downgrade(self)),
            children: RefCell::new(vec![])
        });

        self.children.borrow_mut().push(Rc::clone(&dir));

        dir
    }

    fn add_file(self: &Rc<Directory>, amount: usize) {
        *self.size.borrow_mut() += amount;

        if let Some(parent) = self.parent.borrow().upgrade() {
            update_size(&parent, amount)
        }
    }

    fn get_parent(self: &Rc<Directory>) -> Rc<Directory> {
        self.parent.borrow().upgrade().unwrap()
    }

    fn get_sizes_under(self: &Rc<Directory>, max_size: usize) -> usize {
        let own_size = *self.size.borrow();
        let mut size = if own_size < max_size { own_size } else { 0 };

        for child in self.children.borrow().iter() {
            size += child.get_sizes_under(max_size);
        }
        size
    }
}

fn build_filesystem(content: String) -> Rc<Directory> {
    let root = Directory::new();

    let mut dir = root.clone();

    for line in content.lines().skip(1) {
        if line == "$ ls" {
            continue;
        } else if line.starts_with("dir ") {
            let dir_name = String::from(&line[4..]);
            dir.create_dir(dir_name);
        } else if line == "$ cd .." {
            dir = dir.get_parent();
        } else if line.starts_with("$ cd ") {
            let dir_name = String::from(&line[5..]);
            dir = dir.open_dir(dir_name).unwrap();
        } else {
            let split = line.split(" ").collect::<Vec<&str>>();
            let size = split.get(0).unwrap();
            dir.add_file(size.parse::<usize>().unwrap())
        }
    }

    root
}

fn update_size(directory: &Rc<Directory>, amount: usize) {
    *directory.size.borrow_mut() += amount;

    if let Some(parent) = directory.parent.borrow().upgrade() {
        update_size(&parent, amount)
    }
}

