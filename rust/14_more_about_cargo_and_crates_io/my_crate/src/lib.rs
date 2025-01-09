//! # My crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more
//! convenient.

/// Adds one to the given number.
///
/// # Examples
///
/// ```
/// let args = 5;
/// let answer = my_crate::add_one(args);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: u32) -> u32 {
    x + 1
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
