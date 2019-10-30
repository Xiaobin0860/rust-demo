use std::mem::size_of_val;
use Linklist::{Item, Nil};

#[derive(Debug)]
enum Linklist {
    Item(Box<u32>, Box<Linklist>),
    Nil,
}

pub fn run() {
    println!("---------------");
    //Struct alloc::boxed::Box A pointer type for heap allocation.
    let value = Box::new(8);
    println!("value is {:?}, size: {}", value, size_of_val(&value));
    let node1 = Item(value, Box::new(Nil));
    println!("node1 is {:?}, size: {}", node1, size_of_val(&node1));
    let node2 = Item(Box::new(7), Box::new(node1));
    println!("node2 is {:?}, size: {}", node2, size_of_val(&node2));
    let node = Box::new(node2);
    println!("node is {:?}, size: {}", node, size_of_val(&node));
}
