fn main() {
    let mut x: u8 = 0;

    loop {
        println!("again! {x}");
        x = x + 1;

        if x > 100 {
            break;
        }
    }

    println!("BREAKING!");

    // the thing after the keyword loop is an expression that does have something to return so you
    // can do the below

    let mut counter: u8 = 0;
    let result: u8 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{result}");

    let mut count: u8 = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining: u8 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // breaking up the 'counting_up loop only NOT the innermost loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number: u8 = 10;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("lifeoff!");

    let mut number: u8 = 10;

    'life_off: loop {
        println!("{number}!");

        number -= 1;

        if number == 0 {
            break 'life_off;
        }
    }

    println!("lifeoff!");

    let a: [u8; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for current_element in a {
        println!("!the vlaue is: {}", current_element);
    }

    for current_number in (1..11).rev() {
        println!("{current_number}!");
    }

    let asdf: std::ops::Range<u8> = std::ops::Range { start: 1, end: 11 }; // would be the same as
                                                                           // 1..11 or
                                                                           // (1..11)

    for current_asdf in asdf.rev() {
        println!("{current_asdf}");
    }

    println!("LIFEOFF!");
}
