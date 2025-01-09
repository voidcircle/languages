use add_one::add_one;
use add_two::add_two;

fn main() {
    println!("Hello, World");

    let defalut_value: i32 = 0;

    println!("This is the default value: {defalut_value}");

    let value_with_add_one: i32 = add_one(defalut_value);

    println!("This is the value with the add one function: {value_with_add_one}");

    let value_with_add_two: i32 = add_two(defalut_value);

    println!("This is the value with the add two function: {value_with_add_two}");
}
