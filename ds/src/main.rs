use std::mem;
mod shap;

#[allow(dead_code)]
#[derive(Debug)]
enum Role {
    Developer,
    Designer,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Role::Developer
    }
}

#[derive(Debug, Default)]
struct Employee {
    name: String,
    age: u8,
    role: Role,
}

fn main() {
    show_tuple();
    show_array();
    show_slice();

    let name = String::from("Frank");
    println!("String Length: {}", name.len());
    println!("String Capacity: {}", name.capacity());
    //ptr, len, capacity
    println!("var size: {}", mem::size_of_val(&name));
    let sub = &name[0..1]; //ptr, len
    println!("slice Length: {}", sub.len());
    println!("slice size: {}", mem::size_of_val(&sub));
    let s = "F";
    println!("str Length: {}", s.len());
    println!("str size: {}", mem::size_of_val(&s));
    let mut dev = Employee::default();
    dev.name = name;
    dev.age = 30;
    let admin = Employee::default();
    println!(
        "{:?}, size={}, {:?}",
        dev,
        mem::size_of_val(&dev),
        (
            mem::size_of_val(&dev.name),
            mem::size_of_val(&dev.age),
            mem::size_of_val(&dev.role)
        )
    );
    println!("{:?}, size={}", admin, mem::size_of_val(&admin));

    shap::static_dispatch();
    shap::dynamic_dispatch();
}

fn show_tuple() {
    let tuple = ("hi", 42u128, "world!", [3, 6, 9]);
    println!("First element is {}", tuple.0);
    println!("Second element is {}", tuple.1);
    println!("Third element is {}", tuple.2);
    println!(
        "{:?} size={}, {:?}",
        tuple,
        mem::size_of_val(&tuple),
        (
            mem::size_of_val(&tuple.0),
            mem::size_of_val(&tuple.1),
            mem::size_of_val(&tuple.2),
            mem::size_of_val(&tuple.3)
        )
    );
    let mut counter = 0;
    for x in &tuple.3 {
        println!("Element {} of the fourth element is {}", counter, x);
        counter += 1;
    }
}

fn show_array() {
    let array: [u32; 5] = [0, 1, 2, 3, 5];
    println!("{:?}, size: {}", array, mem::size_of_val(&array));
    for i in array.iter() {
        println!("{}", i);
    }
}

fn show_slice() {
    let array: [i32; 5] = [0, 1, 2, 3, 4];

    let slice = &array[0..4];
    println!("{:?} {}", slice, mem::size_of_val(&slice));
    for x in slice {
        println!("x is {}", x);
    }
}
