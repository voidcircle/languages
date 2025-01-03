fn main() {
    println!("Hello, world!");
    let result: i8 = another_function();
    print_number(result);
    print_number(another_function());

    greet("Rust", "beginner", 16);

    let y: u8 = {
        let mut x: u8 = 3;
        x = x + 4;
        x + 1
    };

    println!("The y is {y}");

    let mut number_five: u8 = get_five();
    println!("{number_five}");
    number_five = plus_one(number_five);
    println!("{number_five}");
}

fn another_function() -> i8 {
    println!("another function!");

    return -2;
}

fn print_number(number: i8) {
    println!("The value is {number}");
}

fn greet(field: &str, job: &str, age: u8) {
    println!("Hi, I am a {field} {job} and I am {age} years old.");
}

fn get_five() -> u8 {
    5
    // return 5; would be the same
}

fn plus_one(x: u8) -> u8 {
    return x + 1;
}
