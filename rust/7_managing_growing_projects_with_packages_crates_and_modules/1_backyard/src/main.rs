use crate::garden::vegetables::Asparagus;

// defining a module
pub mod garden;

fn main() {
    let plant = Asparagus {};

    println!("I am growing {plant:?}!");
}
