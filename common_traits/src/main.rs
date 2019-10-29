use std::fmt;
use std::string::ToString;

mod dv;

#[derive(Debug, Default)]
struct CreditCardInfo {
    number: String,
    exp: String,
    holder: String,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Payment {
    Cash(u32),
    CreditCard(CreditCardInfo),
}

impl Default for Payment {
    fn default() -> Self {
        Payment::Cash(Default::default())
    }
}

impl fmt::Display for CreditCardInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} with EXP {} BY {}",
            self.number, self.exp, self.holder
        )
    }
}

impl ToString for Payment {
    fn to_string(&self) -> String {
        format!("{:#?}", self)
    }
}
fn process() {
    let payment1 = Payment::Cash(100);
    let payment2 = Payment::CreditCard(CreditCardInfo {
        number: String::from("12345678"),
        exp: String::from("10/29"),
        holder: String::from("XIAOBIN"),
    });
    println!("payment1: {}", payment1.to_string());
    println!("payment2: {}", payment2.to_string());

    match payment2 {
        Payment::CreditCard(info) => println!("{} {:?}", info, info),
        _ => println!("Not matched!"),
    }

    println!("change 42 to string is: {}", 42.to_string());

    dv::default_trait();

    let payment3: Payment = Default::default();
    println!("default for payment: {:#?}", payment3);
}
#[derive(Clone, Debug)]
struct Pointer {
    x: u32,
    y: u32,
}

fn show_pointer(p: Pointer) -> String {
    format!("{:?}", p)
}

fn copy_clone_test() {
    let pointer = Pointer { x: 10, y: 20 };
    let result = show_pointer(pointer.clone());
    println!("result is {}, origin pointer is {:?}", result, pointer);
}

fn main() {
    let mut it = [10, 20, 30].iter();
    while let Some(n) = it.next() {
        println!("got {}", n);
    }

    for n in [10, 20, 30].iter() {
        println!("got {}", n);
    }

    println!("{:?}", [10, 20, 30].iter().map(|x| x + 1));
    for n in [10, 20, 30].iter().map(|x| x + 1) {
        println!("got {}", n);
    }
    let v: Vec<_> = [10, 20, 30].iter().map(|x| x + 1).collect();
    println!("{:?}", v);

    let sum: i32 = [1, 2, 3].iter().sum();
    println!("sum={}", sum);

    let total: i32 = vec![1i32, 2, 3]
        .iter()
        .filter(|&&x| x % 2 == 1)
        .zip([4, 5].iter())
        .map(|(a, b)| a * b)
        .sum();
    println!("{:?}", total);
    println!("{:?}", [1i32, 2, 3].iter().filter(|&&x| x % 2 == 1));

    process();

    copy_clone_test();
}
