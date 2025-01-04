fn main() {
    // what is happening?
    //
    // 1. `get_reference_to_nothing` is called
    // 2. inside of that function, new string is created
    // 3. the REFERENCE of the new_string will be returned and only the reference will be defined
    //    to the `reference_to_nothing` variable.
    // 4. BUT! new_string, which is the original data would be GONE since it would have gone out of
    //    scope as soon as the function finishes.
    // 5. Then.... `reference_to_nothing` variable now becomes referencing to literally nothing.
    //
    // The compiler and LSP know that so they are angry at you because you just created a wasted
    // data usage example.
    //
    // The way to solve this problem at the first time is to pass out the ownership too which is
    // also referenced as moving the ownership and return the ownership too.
    //
    // As you know, if you are dealling with SIMPLE data type, this won't happen because Rust will
    // just copy and pass out the copied one. I think it would be a good time explaining what would
    // have happened if the new_string was just a i32 type which is a ismple type.
    //
    // 1. `get_reference_to_nothing` is called
    // 2. Insidee of that function new_string variable but with i32 type will be created
    // 3. It gets returned. But the copied one will get returned while the original data stays
    //    inside of the scope of the function.
    // 4. THe scope(the function) finishes and new_string will be gone. The original one is gone
    //    but the copied is still alive at...
    // 5. `reference_to_nothing` variable will have the same data as the new_string data type.
    // 6. There won't be anything like dangling reference or something like that at all.

    let reference_to_nothing = get_reference_to_nothing();

    println!("{reference_to_nothing}");
}

fn get_reference_to_nothing() -> &String {
    let new_string: String = String::from("Hello");

    &new_string
}
