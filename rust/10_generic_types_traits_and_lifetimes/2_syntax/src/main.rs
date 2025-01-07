#[derive(Debug)]
pub struct Point<T, U = T> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // cannot add it since it is dublicated
    // fn get_x(&self) -> &T {
    //     &self.x
    // }
    fn get_y(&self) -> &U {
        &self.y
    }
    fn mix_up_with<T1, U1>(&self, other: Point<T1, U1>) -> Point<&T, U1> {
        Point {
            x: &self.x,
            y: other.y,
        }
    }
}

impl Point<i8, u8> {
    fn get_x(&self) -> &i8 {
        println!("HAHA! He is getting x!");
        &self.x
    }
}

fn main() {
    let int_point: Point<i8, u8> = Point { x: 20, y: 100 };

    let float_point: Point<f64> = Point { x: 50.6, y: 10.2 };

    println!("{}", int_point.get_x());
    println!("{}", float_point.get_y());
    dbg!(int_point.mix_up_with::<f64, f64>(float_point));
}

fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest: &T = &list[0];

    for current_element in list {
        if current_element > largest {
            largest = current_element
        }
    }

    largest
}

fn get_largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for current_element in list {
        if current_element > largest {
            largest = current_element;
        };
    }

    largest
}

fn get_largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for current_element in list {
        if current_element > largest {
            largest = current_element;
        };
    }

    largest
}
