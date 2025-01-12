fn main() {
    // match
    let x: Option<i32> = Some(-2000);
    match x {
        None => println!("X is nothing!"),
        Some(value) => println!("X is something: {value}"),
    }

    let favorite_color: Option<&str> = None;
    let is_tuesday: bool = false;
    let age: Result<u8, _> = "34".parse();

    // if let, else if let
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let
    let mut stack: Vec<u8> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // it will loop the code as long as the pattern does match
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for
    let v: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let x: u8 = 5;
    let (x, y, z): (u8, u8, u8) = (1, 2, 3);
    // this will not work
    // let (x, y) = (1, 2, 3);

    boo(&(100, 150));
}

// function
fn foo(x: u8) {}

fn boo(&(x, y): &(u8, u8)) {
    println!("current location is: ({x}, {y})");
}
