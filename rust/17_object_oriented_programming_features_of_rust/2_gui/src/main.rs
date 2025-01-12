use gui::{Draw, Screen};

// here are multiple structs that do implement trait Draw.

#[derive(Debug)]
struct Something {
    some_value: u8,
}

impl Draw for Something {
    fn draw(&self) {
        println!("DRAWING! {}", self.some_value);
    }
}

#[derive(Debug)]
struct Something2 {
    some_values: i8,
}

impl Draw for Something2 {
    fn draw(&self) {
        println!("DRRAWWWING!!! {}", self.some_values);
    }
}

fn main() {
    // if we used the generic type to define the Screen struct, then we should have used the screen
    // struct only in this way that can accept only ONE enum, or ONE struct that does implement the
    // trait Draw.
    //
    // let something: Screen<Something> = Screen {
    //     components: vec![
    //         Something { some_value: 10 },
    //     ],
    // };

    // since we define the struct Screen with Box<dyn Draw>, we can define a variable that can
    // accept multiple different structs or enums that do implement the Draw trait.

    let something: Screen = Screen {
        components: vec![
            Box::new(Something { some_value: 20 }),
            Box::new(Something2 { some_values: -100 }),
        ],
    };
}
