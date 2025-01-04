#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // ASSOCIATED FUNCTION! SHOULD BE CALLED
    // `Rectangle::get_square(20)`
    // it does not have the first parameter as the self.
    fn get_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    // method! should be called
    // rectangle.get_area()
    // it DOES have the first parameter as the self.
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
    fn get_perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// a struct can have multiple implementation blocks
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn check_if_can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width >= other_rectangle.width && self.height >= other_rectangle.height
    }
}

fn main() {
    let rectangle1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2: Rectangle = Rectangle {
        width: 80,
        height: 20,
    };

    let rectangle3: Rectangle = Rectangle {
        width: 20,
        height: 10,
    };

    let rectangle4: Rectangle = Rectangle::get_square(100);

    println!("Rectangle's area is {}", rectangle1.get_area());
    println!("Rectangle's perimeter is {}", rectangle1.get_perimeter());
    println!("{}", rectangle1.width()); // rust knows what we mean when we use width <- field
                                        // or width() <- method
    println!("{}", rectangle1.check_if_can_hold(&rectangle2)); // false
    println!("{}", rectangle1.check_if_can_hold(&rectangle3)); // true
    println!("{}", rectangle4.check_if_can_hold(&rectangle2)); // true

    dbg!("{}", rectangle2);
    dbg!("{}", rectangle4);
}
