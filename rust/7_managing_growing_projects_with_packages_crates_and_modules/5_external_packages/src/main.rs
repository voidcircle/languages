// use std::cmp::Ordering;
// use std::io;
//
// would be the same as
//
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
//
// would be the same as
//
// use std::io::{self, Write};
//
// it just brings ALL public items
// use std::collections::*;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
