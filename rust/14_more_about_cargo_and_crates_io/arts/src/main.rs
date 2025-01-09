// use arts::kinds::PrimaryColor;
// use arts::utils::mix;
//
// but if the author of `arts` crate added
// pub use keyword...?

use arts::{mix, PrimaryColor};

fn main() {
    let red: PrimaryColor = PrimaryColor::Red;
    let yellow: PrimaryColor = PrimaryColor::Yellow;
    mix(red, yellow);
}
