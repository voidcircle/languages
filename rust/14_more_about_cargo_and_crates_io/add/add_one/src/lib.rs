pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_one() {
        let value: i32 = 0;

        assert_eq!(1, add_one(value));
    }
}
