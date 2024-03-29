use Linklist::{Item, Nil};

use std::mem::size_of_val;
//Struct alloc::rc::Rc A single-threaded reference-counting pointer. 'Rc' stands for 'Reference Counted'.
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Linklist {
    Item(u32, Rc<Linklist>),
    Nil,
}

impl Drop for Linklist {
    fn drop(&mut self) {
        println!("item to be freed: {:?}", self);
    }
}

pub fn run() {
    println!("---------------");
    let node1 = Item(8, Rc::new(Nil));
    let node = Rc::new(Item(7, Rc::new(node1)));
    {
        let list1 = Item(1, Rc::clone(&node));
        println!(
            "list1: {:?}, size: {}, node rc is: {}",
            list1,
            size_of_val(&list1),
            Rc::strong_count(&node)
        );
        let list2 = Item(2, Rc::clone(&node));
        println!(
            "list2: {:?}, size: {}, node rc is: {}",
            list2,
            size_of_val(&list2),
            Rc::strong_count(&node)
        );
    }

    println!("node rc is: {}", Rc::strong_count(&node));
    println!("node is {:?}, size: {}", node, size_of_val(&node));
}
