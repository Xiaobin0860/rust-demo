#[derive(Debug)]
pub struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    pub fn can_hold(&self, other: &Rect) -> bool {
        other.w <= self.w && other.h <= self.h
    }
}

pub fn add_two(n: i32) -> i32 {
    n + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug, Default)]
pub struct Guess {
    v: i32,
}

impl Guess {
    pub fn new(v: i32) -> Self {
        if v < 1 || v > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", v);
        }
        Guess { v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect { w: 8, h: 7 };
        let smaller = Rect { w: 5, h: 3 };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rect { w: 8, h: 7 };
        let smaller = Rect { w: 5, h: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("lxb");
        assert!(
            result.contains("lxb"),
            "Greeting didn't contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
