#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

//Struct core::cell::RefCell A mutable memory location with dynamically checked borrow rules
use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

pub fn run() {
    println!("---------------");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    {
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
        let mut a1 = value.borrow_mut();
        // let mut a2 = value.borrow_mut();
        *a1 += 10;
        // *a2 += 10;

        println!(
            "a after borrow = {:?}, ref count: {}",
            a,
            Rc::strong_count(&a)
        );
        println!("b after borrow = {:?}", b);
        println!("c after borrow = {:?}", c);
        println!(
            "value after borrow = {:?}, ref count: {}",
            value,
            Rc::strong_count(&value)
        );
    }
    println!("a after = {:?}, ref count: {}", a, Rc::strong_count(&a));
    println!("b after = {:?}", b);
    println!(
        "value: {:?}, ref count: {}",
        value,
        Rc::strong_count(&value)
    );
}
