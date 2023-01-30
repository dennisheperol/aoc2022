use std::cell::RefCell;
use std::fs;
use std::rc::{Rc, Weak};

pub fn part1() {
    let content = fs::read_to_string("puzzle_input/day07.txt").expect("Couldn't read file");

    build_filesystem(content);

    // println!("{content}");
}

pub fn part2() {
    let content = fs::read_to_string("puzzle_input/day07.txt").expect("Couldn't read file");

    println!("{content}");
}

#[derive(Debug)]
struct Directory {
    size: Rc<RefCell<usize>>,
    parent: RefCell<Weak<Directory>>,
    children: RefCell<Vec<Rc<Directory>>>,
}

fn build_filesystem(content: String) {
    let root = Rc::new(Directory {
        size: Rc::new(RefCell::new(0)),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("start");
    println!("{root:?}");
    // add file =====================================

    update_size(&root, 100);

    println!("after file");
    println!("{root:?}");

    //add dir =====================================

    let dir = Rc::new(Directory {
        size: Rc::new(RefCell::new(0)),
        parent: RefCell::new(Rc::downgrade(&root)),
        children: RefCell::new(vec![])
    });

    root.children.borrow_mut().push(Rc::clone(&dir));

    println!("after dir");
    println!("{root:?}");

    //add dir in dir =====================================

    let sub_dir = Rc::new(Directory {
        size: Rc::new(RefCell::new(0)),
        parent: RefCell::new(Rc::downgrade(&dir)),
        children: RefCell::new(vec![])
    });

    dir.children.borrow_mut().push(Rc::clone(&sub_dir));

    let dir = sub_dir;

    println!("after sub sir");
    println!("{root:?}");

    // add file in subdir =====================================

    update_size(&dir, 50);

    println!("after sub dir update");
    println!("{root:?}");

    // cd .. and add new subdir
    let dir = dir.parent.borrow().upgrade().unwrap();

    let sub_dir = Rc::new(Directory {
        size: Rc::new(RefCell::new(0)),
        parent: RefCell::new(Rc::downgrade(&dir)),
        children: RefCell::new(vec![])
    });

    dir.children.borrow_mut().push(Rc::clone(&sub_dir));

    let dir = sub_dir;
    update_size(&dir, 75);

    println!("after new sub dir with file");
    println!("{root:?}");
}

fn update_size(directory: &Rc<Directory>, amount: usize) {
    *directory.size.borrow_mut() += amount;

    if let Some(parent) = directory.parent.borrow().upgrade() {
        update_size(&parent, amount)
    }
}

