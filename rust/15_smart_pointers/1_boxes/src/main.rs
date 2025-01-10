#[derive(Debug)]
enum List {
    // Cons(i32, List), // in this, it will not be compiled since Rust does not know how much space
    // List is going to take up.
    //
    //
    // in this way, Rust knows that List enum will take up the space of the size of i32 and the
    // size of the pointer.
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn asdf(&self) -> i32 {
        100
    }
}

use crate::List::{Cons, Nil};

fn main() {
    // a Box type is a smart pointer that points to the data that is stored at the heap.
    let b: Box<i32> = Box::new(10);
    println!("{b}");

    let list = Cons(1, Box::from(Cons(2, Box::from(Cons(3, Box::new(Nil))))));
}
