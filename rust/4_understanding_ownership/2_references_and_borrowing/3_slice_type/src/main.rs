fn main() {
    let original_string = String::from("Hello World");
    let first_word: &str = find_the_first_word(&original_string);
    let _ss1 = &original_string[..5]; // would be the same as 0..5
    let _ss2 = &original_string[5..]; // would be the same as 5..original_string.len()
    let _ss3 = &original_string[5..original_string.len()];

    println!("{original_string}");
    println!("{first_word}");

    // the below has the type of `&str` notice that the type has the letter ampercent which means
    // it does NOT hold the data right away but the vraible holds the data representing where to
    // point to. That is why it is IMMUTABLE(you canNOT edit it) because it does not own it; it
    // just references to it.
    let _some_string = "Hello World";

    // below is the code that lived in the docs
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let _word = find_the_first_word(&my_string[0..6]);
    let _word = find_the_first_word(&my_string[..]);
    // `find_the_first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = find_the_first_word(&my_string);

    let my_string_literal = "hello world";

    // `find_the_first_word` works on slices of string literals, whether partial or whole
    let _word = find_the_first_word(&my_string_literal[0..6]);
    let _word = find_the_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = find_the_first_word(my_string_literal);

    let a: [u8; 5] = [1, 2, 3, 4, 5];
    let b: &[u8] = &a[1..3];
    let are_equal = &[2, 3] == b;
    println!("{are_equal}"); // true
}

// Notice that the original_string: &str; WE do not use &String because it onyl receives &String
// type only not &str but by putting there &str there, it can now receives &String type data and
// &str type data
fn find_the_first_word(original_string: &str) -> &str {
    let bytes = original_string.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &original_string[0..index];
        }
    }

    return &original_string[..];
}
