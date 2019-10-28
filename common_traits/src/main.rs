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
}
