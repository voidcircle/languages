use std::io;

fn main() {
    let number: u8 = 12;

    if number < 5 {
        println!("the condition was true.");
    } else {
        println!("the condition was false.");
    }

    if number != 0 {
        println!("the number is not zero.");
    } // the block of code right after the if statement is a expression, which you can also write
      // like below since expressions are block of codes that do have something to return.

    let is_number_big_enough: bool = if number >= 10 {
        let x: bool = true;
        x
    } else {
        let x: bool = false;
        x
    };

    // but the below is more... better.

    let condition: bool = number > 10;
    let is_number_big_enough: u8 = if condition { 5 } else { 200 }; // of course, each `ARM` should
                                                                    // return the same type of data

    println!("{is_number_big_enough}");
}
