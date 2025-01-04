fn main() {
    // explanation of what is happening below.
    // 1. s1 is created. COMPLEX!
    // 2. `calculate_length` is called with s1 MOVED. s1 won't be valid since it is a complex data
    //    type which won't be copied by Rust.
    // 3. inside of that `calculate_length` function, length is created with a SIMPLE data type.
    // 4. length will be COPIED and the copied one will be returned and s will be MOVED(NOT COPIED)
    // 5. length meets the end of the scope of that function `calculate_length` and will be FREED!
    // 6. s2 gets assigned with the same moved ownership as s1 and len will get assigned with the
    //    simple type `usize`
    // 7. prints blalbalb
    // 8. s2 and len meets the end of the scope which will free those variables.

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    // println!("{s1}"); // impossible
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length: usize = s.len(); // len() returns the length of a String

    (s, length)
}
