fn main() {
    let s: String = String::new();

    println!("{s}");

    let ss: String = "Hello, this is a string literal &str".to_string();

    let sr: &str = "Hello, this is a string literal too";
    sr.to_string();

    println!("{ss}");
    println!("{sr}");

    let mut sf: String = String::from("Hello, this is a String!");

    println!("{sf}");

    sf.push_str(" Hi, this is an added string!");
    sf += " Hi, this is another added string!";
    sf = format!("{} {}", sf, " Hi, this is another another added string!");

    println!("{sf}");

    let mut sf2: String = String::from("Hello, this is a STRING!");
    let sf3: &str = "This is a string literal.";

    // push_str method does NOT take the ownership.
    sf2.push_str(sf3);

    println!("{sf2} || {sf3}");

    let sf4: String = String::from("HIHIHIHI");

    // but + operator DOES take the ownership.
    sf2 += &sf4;

    println!("{sf2} || {sf4}");

    let s10 = String::from("tic");
    let s20 = String::from("tac");
    let s30 = String::from("toe");

    // In this case, s10 will lose its ownership but not s20 and s30 since they are just
    // referencing
    // let s = s10 + "-" + &s20 + "-" + &s30;
    // println!("{s}, {s10}, {s20}, {s30}");

    let s = format!("{s10}-{s20}-{s30}");
    println!("{s}");
    println!("{s10}, {s20}, {s30}");

    // it is not allowed to access to String character by character.
    //
    // let sss = &s[1]; // or
    // let sss = &s[2..3]
    //
    // This is not allowed due to the way that Rust handles the data that represents strings and
    // letters. More https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings

    for current_letter in s.chars() {
        println!("{current_letter}");
    }

    for current_byte in s.bytes() {
        println!("{current_byte}");
    }
}
