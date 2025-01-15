use std::{fmt::Display, ops::Add};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// we can define the generic type!!!! and fo rthe generic types, we can make multiple
// implementaitons for one struct while the associated types, we do not have to.
impl<'a> Add<&'a Point> for &'a Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<i32> for Point {
    type Output = Point;

    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

/////////////////////////////

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Human {
    pub fn fly(&self) {
        println!("HUMAN FLY!");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("PILOT FLY!");
    }
}

pub struct Alien;

impl Wizard for Human {
    fn fly(&self) {
        println!("WIZARD FLY!");
    }
}

/////////////////////////////

pub trait Animal {
    fn get_baby_name() -> String;
}

pub struct Dog;

impl Dog {
    fn get_baby_name() -> String {
        String::from("SPOT!")
    }
}

impl Animal for Dog {
    fn get_baby_name() -> String {
        String::from("Spot")
    }
}

/////////////////////////////

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output: String = self.to_string();
        let output_len = output.len();

        println!("{}", "*".repeat(output_len + 4));
        println!("*{}*", " ".repeat(output_len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(output_len + 2));
        println!("{}", "*".repeat(output_len + 4));
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

/////////////////////////////

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let point_1 = Point { x: 10, y: 82 };
    let point_2 = Point { x: 25, y: 13 };

    // Operator like + has NOT beem implemented on the basic struct, but by simply adding the Add
    // trait, we can do this! Wow!
    //
    // Other than that, we can do Sub, Div, Mul, SubAssign(-=), DivAssign(/=), and etc...
    println!("{:#?}", &point_1 + &point_2);

    point_1.outline_print();

    let human_1 = Human;

    // even those there are dubplicated methods, Rust calls the method that is implemented directly
    // on the instance's struct.
    human_1.fly(); // the same as Human::fly(&human_1); but why would we write code in this way?

    // it does make sense... well because what Pilot and Wizar traits need is the datat that is in
    // the shape of human.
    Pilot::fly(&human_1);
    Wizard::fly(&human_1);

    // even those there are dubplicated associated methods, Rust calls the associated method that
    // is implemented directly on the instance's struct 2222
    println!("{}", Dog::get_baby_name());

    // if we want to call the get_baby_name associated method on the Animal trait...  then
    // If we did not put <Dog as Animal> thing, then we will get a compiler error because Rust
    // cannot figure out which associated method to call(since a trait can be implemented on
    // multiple structs)
    println!("{}", <Dog as Animal>::get_baby_name());

    let w = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("w = {w}");
}
