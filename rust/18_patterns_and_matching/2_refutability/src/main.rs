fn main() {
    // irrefutable because x can be bound into anything no matter what the expression is!
    let x: u8 = 4;

    // refutable because in the case when the variable x is None as shown below, the variable
    // y cannot be bound forever!
    let x: Option<u8> = None;
    if let Some(y) = x {}

    // Function parameters, let statements, and for sstatements can ONLY accept irrefutable(of
    // course,). if let or while let statements can accept both refutable and irrefutable since
    // they are meant to control possible failures.

    // even though it might make sense in our brains, this will NOT compile because the let
    // statement consiste of `let PATTERN = EXPRESSION` and Some(x) is NOT an irrefutable pattern, that is why
    // it cannot be compiled. There is nothing Rust can do when the value is None.
    // let Some(x) = Some(20);
    // let Some(x) = None; // Rust: WHAT!

    // so, we use if let statement where can accept refutable pattern.
    let some_option_value: Option<u8> = None;
    if let Some(x) = some_option_value {
        println!("{x}");
    }

    // the compiler will NOT give your a compiler error, but will give you a compiler WARNING since
    // if let statement is meant to give your program a chance to cover when failed, but in this
    // example, there is NOTHING Rust can do in failure becasue there is NO possible failures.
    if let x = 20 {
        println!("{x}");
    }
}
