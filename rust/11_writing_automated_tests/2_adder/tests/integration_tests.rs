// if we want to run specific test only in the tests folder, then
// `cargo test --test {file name}`

use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}
