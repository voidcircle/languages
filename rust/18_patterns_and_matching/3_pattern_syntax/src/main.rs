struct Point {
    x: u8,
    y: u8,
}

struct THREED_Point {
    x: u8,
    y: u8,
    z: u8,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // it just shadows...
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 1;

    match x {
        // if x is matched into those two patterns
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        // inclusive range
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // !!!!!!!
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p;

    // the match expression stops check if the next arm is matched as soon as it finds something
    // that is matched.
    match p {
        Point { x, y: 0 } => println!("IT IS ON X AXIS! {x} {y}"),
        Point { x: 0, y } => println!("IT IS ON Y AXIS! {x} {y}"),
        Point { x: 10, y: 11 } => println!("x: 10, y: 11 {x} {y}"),
        Point { x: 11, y: 10 } => println!("x: 10, y: 11 {x} {y}"),
        Point { x, y } => println!("{x} {y}"),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
    }

    // what..?
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });

    foo(20, -20);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // while _{name} does get bound with some value, just _ does NOT.
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // This one moved! s is moved to _
    // println!("{s:?}");
    //
    // while...
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    // _ does NOT BIND AT ALL!
    // So, it was not moved.
    println!("{s:?}");

    let THREED_Point { y: _y, .. } = THREED_Point {
        x: 20,
        y: 160,
        z: 20,
    };

    let origin: THREED_Point = THREED_Point {
        x: 20,
        y: 160,
        z: 20,
    };

    match origin {
        THREED_Point { x: 20, .. } => println!("X IS ON {x}"),
        _ => (),
    };

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        } // (.., second, ..) => { // Rust would not know where...
          //     println!("Some numbers: {second}")
          // },
    }

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
