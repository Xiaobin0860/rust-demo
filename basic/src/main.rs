fn main() {
    // vars are immutable by default, rust is a block-scoped language
    // integer: 127u8, i8, ..., u128, i128
    // Number literals: Dec 100_000, Hex 0xff, Oct 0o77, Binary 0b1_0000, Byte(u8) b'A'
    // floats: f32, f64
    // boolean: bool true, false
    // charater: utf8 ''
    // tuple: ()
    // array: []

    let age = 30;
    //age = 31; //cannot assign twice to immutable variablerustc(E0384)
    let name = "frank";
    let mut x = 2.0; //f64
    x = x * 1.2;
    let y: f32 = 3.0;
    const ID: i32 = 001;
    let a = [1, 2, 3];
    let heart = '❤';
    // reuse var name
    let a = (x, y, a, heart);
    //frank is 30 and frank likes rust! ID=1, age=30, Hex: 0x1E
    println!(
        "{} is {} and {0} likes rust! ID={id}, age={1}, Hex: 0x{1:X}",
        name,
        age,
        id = ID
    );
    let gt30 = if age > 30 { true } else { false };
    //a=(2.4, 3.0, [1, 2, 3], '❤'), age+1=31, plus_two=32, gt30=false
    println!(
        "a={:?}, age+1={}, plus_two={}, gt30={}",
        a,
        plus_one(age),
        plus_two(age),
        gt30
    );
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
fn plus_two(x: i32) -> i32 {
    x + 2
}
