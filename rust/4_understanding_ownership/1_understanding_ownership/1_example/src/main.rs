fn main() {
    // println!("{s}");
    {
        // println!("{s}");
        let mut s: &str = "hello";
        println!("{s}");
        s = "Hello";
        println!("{s}");
    }
    // println!("{s}");

    let mut st: String = String::from("hello");
    println!("{st}");
    st.push_str(", world!");

    let s1: String = String::from("!hello world!");
    let s2 = s1.clone();

    println!("{s1}");
    println!("{s2}");

    // ss1 has its own data on the stack with the pointer that points at the data on the heap
    // saying "~!hello"
    let mut ss1: String = String::from("~!hello");
    println!("{ss1}");

    // but as soon as the new complex data is assigned to the same variable, Rust will
    // automatically drops the actual content on the heap saying "~!hello" and create another and
    // EDIT the pointer and other data on the stack that this variable holded before.
    ss1 = String::from("Hello!");
    println!("{ss1}");

    let mut x: u8 = 10;
    let mut y: u8 = x;
    x = 8;
    println!("{x} | {y}");
    y = 12;
    println!("{x} | {y}");
}
