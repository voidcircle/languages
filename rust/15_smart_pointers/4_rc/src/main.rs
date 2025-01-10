#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    let b: List = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
    {
        let c: List = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));

    // We do not have to call Rc::decrement_strong_count(&a) <- like this since the drop traits
    // already implemented the feature of decreasing the strong count of the Rc stuff
    // automatically as soon as the variable itself goes out of scope.
}
