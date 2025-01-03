use std::io;

fn main() {
    println!("This is a temperature convert between Celsius and Fahrenheit. If you want to quit from this loop, you can simply type the letter \"q\" at anytime.");

    'convert_temperature: loop {
        let temperature: f64 = {
            let temperature_user_input: String =
                get_user_input("Put the number only without any spaces or commas. Like 32 or 83.")
                    .trim()
                    .to_string();

            if temperature_user_input.eq_ignore_ascii_case("q") {
                break 'convert_temperature;
            }

            match temperature_user_input.parse::<f64>() {
                Ok(num) => num,
                Err(_error_message) => {
                    print!("Unable to convert into a number. Please put the number ONLY. You will be asked to put the unit later. ");
                    continue;
                }
            }
        };

        let unit: String = get_user_input("Put the unit letter only. Like C or f.")
            .trim()
            .to_uppercase();

        if unit.eq_ignore_ascii_case("q") {
            break 'convert_temperature;
        }

        match unit.as_str() {
            "C" => println!("{temperature}C is {}F", convert_from_c_to_f(temperature)),
            "F" => println!("{temperature}F is {}C", convert_from_f_to_c(temperature)),
            _ => unreachable!(),
        }
    }

    println!("Thank you for using the temperature converter.")
}

fn get_user_input(instruction: &str) -> String {
    println!("{instruction}");

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    user_input
}

fn convert_from_c_to_f(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}

fn convert_from_f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}
