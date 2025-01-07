#[derive(Debug)]
struct ImportantExcept<'a> {
    part: &'a str,
}

impl<'a> ImportantExcept<'a> {
    fn do_something(first_arg: &'a str) -> &str {
        "asdfgjkl"
    }
    fn level(&self) -> i32 {
        100
    }
}

fn main() {
    let x: String = String::from("Hello World");
    let result: &str;
    let asdf: &'static str = "This is a string literal. In the case of string literals, they are set to the static lifetime by default, which means those data is stored in the binary.";

    let hahapart: &str = "Hello";

    {
        let y: String = String::from("! I am Seol SO");
        result = longest(x.as_str(), y.as_str());
        println!("{result}");

        // it is possible because as soon as the  asdf goes out of scope,
    }
    let asdf: ImportantExcept = ImportantExcept { part: hahapart };

    // x has NOT gone out of scope, but since y has, the reuslt variable will have the lifetime of
    // y, which is the shorter one.
    // println!("{result}");
}

// &i32 -> a reference to the data of i32
// &'a i32 -> a reference to the data of i32 that has the lifetime of a
// &'a mut i32 -> a mutable reference to the data of i32 that has the lifetime of a

fn longest<'a>(first_str: &'a str, second_str: &'a str) -> &'a str {
    if first_str.len() > second_str.len() {
        first_str
    } else {
        second_str
    }
}

// this one does compile even though the second str is not marked with the lifetime
fn return_first_element<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &str {
    first_str
}

// this one will NOT compile since the lifetime is marked for the return time but has nothing to do
// with the parameters
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
//

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
