fn main() {
    // explanation of what is happening below.
    // 1. `gives_ownership` is called
    // 2. Inside of that function, `some_string` is created and it got returned.
    // 3. `some_string` will be MOVED OUT of the function and kind of re-assigned back to s1. Since
    //    is NOT a simple data type, it WON'T BE COPIED but be moved which means the data on the
    //    stack will be moved and re-assigned back to s1 pointing at the same address at the heap.
    // 4. s2 is created with a compelx data type.
    // 5. `takes_and_gives_back` is called.
    // 6. s2 is passed to the function `takes_and_gives_back` and will be MOVED to the scope that
    //    is located inside of that function. So, s2 will hold nothing meaning it is freed!
    // 7. `takes_and_gives_back` returns exactly what it receives. the ownership(the data on the
    //    stack) will be MOVED again to s3!
    // 8. s1 and s3 meets the end of the scope meaning they will be gone to!

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3

    println!("{s1}");
    // println!("{s2}"); // impossible
    println!("{s3}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
