pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}
