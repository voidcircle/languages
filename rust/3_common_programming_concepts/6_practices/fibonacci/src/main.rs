use std::io;

fn main() {
    let mut current_number: u128 = 1;
    let mut next_number: u128 = 1;
    let mut placeholder: u128;

    let mut current_stage: u8 = 1;
    let max_stage: u8 = get_user_input();

    while current_stage <= max_stage {
        println!("{current_stage}: {current_number}");

        placeholder = current_number;
        current_number = next_number;
        next_number = placeholder + next_number;

        current_stage += 1;
    }
}

fn get_user_input() -> u8 {
    loop {
        println!("Put a singler number");

        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        match user_input.trim().parse::<u8>() {
            Ok(number) => break number,
            Err(_error_message) => {
                println!("Please put one single natural number, not anything else.")
            }
        }
    }
}
