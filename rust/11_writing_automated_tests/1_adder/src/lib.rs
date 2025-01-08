#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Guess {
    fn new(default_value: i32) -> Guess {
        if default_value < 1 {
            panic!("Guess value must be less than or equal to 100, got {default_value}.");
        } else if default_value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {default_value}.");
        }
        Guess {
            value: default_value,
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

pub fn add_greeting(name: &String) -> String {
    // format!("Hello, {name}")
    format!("Hello, ")
}

pub fn some_function(a: i32) -> i32 {
    println!("HEYYYY!");
    a * 20
}

#[cfg(test)]
mod tests {
    use super::*;

    // cannot use #[should_panic] annotation on the functions that return Result
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        // panic!("Make this test fail.");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect1: Rectangle = Rectangle {
            width: 200,
            height: 500,
        };

        let rect2: Rectangle = Rectangle {
            width: 100,
            height: 250,
        };

        assert!(rect1.can_hold(&rect2))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let rect1: Rectangle = Rectangle {
            width: 200,
            height: 500,
        };

        let rect2: Rectangle = Rectangle {
            width: 100,
            height: 250,
        };

        assert!(!rect2.can_hold(&rect1))
    }

    #[test]
    fn get_area_method() {
        let rect1: Rectangle = Rectangle {
            width: 500,
            height: 15,
        };

        assert_eq!(rect1.get_area(), 7500); // ==
        assert_ne!(rect1.get_area(), 6500); // !=
    }

    #[test]
    fn get_greeting() {
        let name = String::from("Seol SO");

        // assert!(
        //     add_greeting(&name).contains("Seol SO"),
        //     "Greeting did not contain name. The value was `{name}`"
        // );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn guess_in_range() {
        // let new_guess: Guess = Guess::new(200); // will cause an error since the test exepctes
        // the error message to incdlue the value for `expected`.
        let new_guess: Guess = Guess::new(-32);
    }

    // We can limit the number of threads by adding a flat --test-threads={integer} since the tests
    // are run in parellaualwejf by default.
    //
    // Everthing that is printed using println! macro will be displayed when the tests are failed.
    // When they are passed, the prints are not displayed.
    // Unless we add a flag --show-output
    //
    // We can search and run specific tests that we want to run by passing them arguments
    // If we run this command `cargo run add` then two tests will be run while
    // if you run this command `cargo run one_hundred` then just onet est will be run since it is
    // the only that matches with the search argument
    // In the case of searching and running specific tests, we will see the `filtered out` value
    // increased.
    //
    // You can ignore some tests and the only way to learn those IGNORED tests only is to run the
    // command `cargo test -- --ignored`
    // If you want to run all tests no matter whether or not they are ignored then you can run the
    // command `cargo test -- --include-ignored`

    #[test]
    fn test_will_pass() {
        let some_variable: i32 = 100;
        assert_eq!(some_function(some_variable), 2000);
    }

    #[test]
    #[ignore]
    fn test_will_fail() {
        let some_variable: i32 = 110;
        assert_eq!(some_function(some_variable), 2000);
    }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        assert!(
            false,
            "THIS ONE WILL BE IGNORED UNLESS SPECIFICALLY MENTIONED"
        );
    }
}
