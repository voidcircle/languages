#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale: u32 = 2;
    let rectangle: Rectangle = Rectangle {
        width: dbg!(180 * scale),
        height: 200,
    };

    let area: u32 = calculate_area(&rectangle);
    let perimeter: u32 = calculate_perimeter(&rectangle);

    println!("Width: {}", rectangle.width);
    println!("Height: {}", rectangle.height);
    println!("Area: {area}");
    println!("Perimeter: {perimeter}");
    println!("{rectangle:#?}");

    dbg!(&rectangle); // dbg! takes the ownership unlike println! which takes the reference only
                      // And... it also returns the ownership that it receives?
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn calculate_perimeter(rectangle: &Rectangle) -> u32 {
    (rectangle.width + rectangle.height) * 2
}
