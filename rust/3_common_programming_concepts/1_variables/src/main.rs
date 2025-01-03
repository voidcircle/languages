use std::io;

fn main() {
    let mut x: u8 = 5; // mutable
    println!("The value of x is: {x}");
    x = 6; // changed
           // x = "123"; // impossible; the variable defined with the mut keyword cannot be mutted
           // with different types
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3; // constants; never change, and can be shared
                                                     // between any scopes
    println!("The number of seconds in three hours is {THREE_HOURS_IN_SECONDS}");

    let a: u8 = 5; // shadowing (shadowed)

    let a: u8 = a + 1; // shadows the original a

    {
        let a: u8 = a * 2; // in the scope, shadowing a which was 6; it will become 12
        println!("The value of a in the inner scope is: {a}");
    }

    // out of the scope, a becomes 6 again
    println!("The value of a is: {a}");

    let spaces: &str = "          "; // shadowing allows the variable to be redefined different types
    let spaces: usize = spaces.len();

    // if you define the variable `spaces` with the mut keyword, then it will NOT be possible to
    // re-define it with different types without let keyword since the mut keyword lets the
    // variable have the type that it was defined with
    //
    // But if you shadow it like above, you will be able to redefine it with different types since
    // you are just re-define it.

    println!("The number of spaces: {spaces}");

    let _guess: u8 = "254".parse().expect("Not a number!"); // 2**8
    let _guess: u16 = "400".parse().expect("Not a number!"); // 2**16
    let _guess: f32 = 3.14;
    let _guess: f64 = 1300.23456543029; // f64 is default since modern CPUs can handle both of the types
    let _sum: u8 = 2 + 3; // will always be positive which the sign does not have to be
    let _sub: i8 = 2 - 3; // can be negative which the sign at the fromt should be added(signed)
    let _pro: u8 = 12 * 14;
    let _quo: f32 = 56.7 / 32.2;
    let _tru: i8 = -5 / 3; // Results in -1
    let _rem: u8 = 43 % 5;
    let _t: bool = true;
    let _f: bool = false;
    let _c: char = 'S';
    let _c: char = 's';
    let _c: &str = "asjfklf";
    let _tup: (i16, u32, char) = (-230, 80000, 'A'); // binding
    let (_f, _s, _t): (i16, u32, char) = _tup; // unstructuring
    let _first: i16 = _tup.0;
    let _second: u32 = _tup.1;
    let _third: char = _tup.2;
    let _unit: () = ();
    let _array: [u8; 5] = [1, 2, 3, 4, 5]; // the elements should have the same types and the array in rust
                                           // has fixed length
    let _months: [&str; 12] = [
        // we know that the number of elements would not be changed at all and have the
        // same type
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _array: [u8; 3] = [1, 2, 3]; // can define it with types.
    let _array: [u8; 6] = [3; 6]; // will be [3, 3, 3, 3, 3, 3]
    let _first: u8 = _array[0]; // 3
    let _second: u8 = _array[1]; // 3
    let _third: u8 = _array[2]; // 3

    let array: [i8; 6] = [-1, 2, -3, 4, 5, 6];

    println!("Please enter an index.");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not an integer.");

    let element: i8 = array[index];

    println!("The element in the array at the index of {index} is {element}");
}
