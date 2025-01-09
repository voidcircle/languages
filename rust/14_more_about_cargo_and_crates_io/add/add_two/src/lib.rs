pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two() {
        let value: i32 = 0;
        assert_eq!(2, add_two(value));
    }
}
