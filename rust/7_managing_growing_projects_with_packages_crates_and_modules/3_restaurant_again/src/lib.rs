mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// as!
use crate::front_of_house::hosting as asdf;
use std::collections::HashMap;

mod customer {
    // wow...
    // note that you can use super
    use super::front_of_house::hosting as asdf;

    pub fn eat_at_restaurant() {
        asdf::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    asdf::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}

// this could have been written like
// use std::fmt::Result;
// use std::io::Result;
//
// But this won't be allowed since there are two different types with the same name.
// When we call them, Rust  might be confused.

use std::fmt;
use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }
//
// fn function2() -> io::Result<()> {
//     // --snip--
// }
