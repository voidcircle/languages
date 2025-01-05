fn main() {
    let five = Some(5);
    let maybe_four: Option<i32> = None;

    dbg!(plus_one(five));
    dbg!(plus_one(maybe_four));

    roll_dice(3);
    roll_dice(10);
    roll_dice(7);
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn roll_dice(number: u8) {
    match number {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // you can also pass the extra variable
    match number {
        3 => remove_fancy_hat(),
        7 => add_fancy_hat(),
        other => move_it(other),
    }
}

fn add_fancy_hat() {
    println!("You got a fancy hat...?");
}

fn remove_fancy_hat() {
    println!("You lost a fancy hat...?"); // what is a fancy hat tho?
}

fn move_it(num_spaces: u8) {
    for _ in 0..num_spaces {
        println!("MOved!")
    }
}
