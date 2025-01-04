fn main() {
    // explanation of what is happening below.
    // 1. s is created with the data on the stack and heap. The data on the stack is pointer,
    //    length, and capacity and the data on the heap holds the actual contents, which is hard to
    //    control over.
    //  2. `takes_ownership` function literally takes everything of s. The data on the stack and
    //     the heap moves into the scope of that function. So, it is INVALID after that function in
    //     the scope where s was created since... everything was taken.
    //  3. x is created with the data on the stack only since 5, which is i32(or u8), is a simple
    //     data type which is EASIER to control.
    //  4. `makes_copy` function does not take everything of x but makes a copy of it and takes the
    //     copied one. Why? Becuase Rust things that the data with those data type is easier to
    //     control so just copying everything there and takes the copied one and put the copied one
    //     into the scope of the function `makes_copy`. As soon as that function ends, the variable
    //     with the copied one goes out of scope, so the COPIED ONE IS DROPPED. But, since the
    //     original data is sitting on the scope where it was created, it is still VALID even after
    //     the function `makes_copy` is called.
    //  5. x is printed
    //  6. the scope ends; the original x goes out of scope so it is gone.

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{s}"); // unable

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    println!("{x}"); // able
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.}
