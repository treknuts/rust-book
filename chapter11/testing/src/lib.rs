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

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
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

        assert_eq!(big.can_hold(&small), true);
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

        assert_eq!(r1.can_hold(&r2), true);
    }
}
