use std::{rc::Rc, borrow::BorrowMut, cell::RefCell, default, collections::HashMap};
use crate::List::*;

#[derive(Debug)]
struct FileNode {
    name: String,
    node_type: bool,
    entries: Vec<FileNode>,
    parent: Option<Rc<RefCell<FileNode>>>
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    q: RefCell<HashMap<String, u64>>
}

fn main() {
    let p = Point { x: 4, y: Default::default(), q: RefCell::default() };

    println!("{:?}", p);

    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));
}
