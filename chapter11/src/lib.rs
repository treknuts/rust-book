#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess must be less than or equal to 100, got {}", value);
        }
        Guess { value }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(i: i32) -> i32 {
    i + 2
}

pub fn greet(name: &String) -> String {
    format!("Greetings, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn adder_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expect = "less than or equal to 100")]
    fn guess_greater_than_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expect = "greater than or equal to 1")]
    fn guess_less_than_1() {
        Guess::new(101);
    }

    #[test]
    fn can_hold_smaller() {
        let big = Rectangle {
            width: 12,
            height: 6,
        };

        let small = Rectangle {
            width: 10,
            height: 5,
        };

        assert!(big.can_hold(&small));
    }

    #[test]
    fn can_hold_same() {
        let r1 = Rectangle {
            width: 5,
            height: 5,
        };

        let r2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn cannot_hold_larger() {
        let larger = Rectangle {
            width: 100,
            height: 23,
        };

        let smaller = Rectangle {
            width: 12,
            height: 3,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn two_plus_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn two_plus_two_wrong() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greet_test() {
        let name = String::from("treknuts");
        let greeting = greet(&name);
        assert!(
            greeting.contains(&name),
            "The greeting did not contain {}",
            &name
        );
    }
}
