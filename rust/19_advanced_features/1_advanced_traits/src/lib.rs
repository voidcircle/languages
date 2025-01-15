use std::ops::Add;

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
    // differences between the associated type and the generics is that
    // the associated type should be defined on each implementation while the concrete type for generics
    // is defined when an instance is made.
    //
    // impl Iterator<T> for Counter
    //
    // means that there can be Iterator<i32> or Iterator<bool> and other implementations. But in
    // the case of impl Iterator for Counter with associated types, there should be one and.. we do
    // not even have to specify for other things.
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(20)
    }
}

/////////////////////////////

struct Milimeters(u32);

impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Milimeters(self.0 + (rhs.0 / 1000))
    }
}

struct Meters(u32);

impl Add<Milimeters> for Meters {
    type Output = Meters;

    fn add(self, rhs: Milimeters) -> Self::Output {
        Meters(self.0 + (rhs.0 * 1000))
    }
}
