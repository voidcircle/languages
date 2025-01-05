fn main() {
    let config_max: Option<i32> = None; // or Some(30)

    if let Some(max) = config_max {
        println!("{max}!");
    } else {
        println!("Wow!");
    }

    // the smae as

    match config_max {
        Some(max) => println!("{max}!"),
        _ => println!("Wow!"),
    }
}
