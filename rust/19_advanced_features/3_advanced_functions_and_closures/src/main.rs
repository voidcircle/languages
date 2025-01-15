fn add_one(args: i32) -> i32 {
    args + 1
}

fn do_twice(f: fn(i32) -> i32, args: i32) -> i32 {
    f(args) + f(args)
}

// when returning a closure, you have to let Rust know how much space will take up but since
// closures are traits, Rust treats it as a Dynamically Sized Type, which means you have to put it
// in smart pointers or reference to the function.
fn _return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let the_number: i32 = 5;

    println!("{}", do_twice(add_one, the_number));
}
