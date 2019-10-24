use std::ops;

trait Value: ops::Mul<Output = Self> + ops::Mul<f64, Output = Self> + Sized + Copy {}
trait Shape<T: Value> {
    fn area(&self) -> T;
}

struct Rect<T> {
    width: T,
    height: T,
}
struct Circle<T> {
    radius: T,
}
struct Square<T> {
    width: T,
}

impl Value for f64 {}

impl<T: Value> Shape<T> for Rect<T> {
    fn area(&self) -> T {
        self.width * self.height
    }
}

impl<T: Value> Shape<T> for Circle<T> {
    fn area(&self) -> T {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl<T: Value> Shape<T> for Square<T> {
    fn area(&self) -> T {
        self.width * self.width
    }
}
pub fn static_dispatch() {
    let rec: Rect<f64> = Rect {
        width: 10.0,
        height: 20.0,
    };
    let circle: Circle<f64> = Circle { radius: 10.0 };
    let square: Square<f64> = Square { width: 10.0 };

    println!(
        "rec area is {}, circle area is {}, square area is {}",
        rec.area(),
        circle.area(),
        square.area(),
    );
}

pub fn dynamic_dispatch() {
    let shapes: [&Shape<f64>; 3] = [
        &Rect {
            width: 10.0,
            height: 20.0,
        },
        &Circle { radius: 10.0 },
        &Square { width: 10.0 },
    ];
    for (i, shape) in shapes.iter().enumerate() {
        println!("{} area {}", i, shape.area());
    }
}
