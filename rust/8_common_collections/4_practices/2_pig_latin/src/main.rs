fn main() {
    let original_string: String = String::from("first");
    let pig_latin: String = convert_to_pig_latin(&original_string);

    println!("Original String: {original_string}\nPig Latin: {pig_latin}");
}

fn convert_to_pig_latin(string: &String) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut pig_latin: String = String::new();

    for current_word in string.to_lowercase().split_whitespace() {
        let first_letter: Option<char> = current_word.chars().nth(0);

        match first_letter {
            Some(fl) => {
                if VOWELS.contains(&fl) {
                    pig_latin.push_str(&format!("{current_word}-hay"));
                } else {
                    let rest_word = current_word.to_string().split_off(1);
                    pig_latin.push_str(&format!("{rest_word}-{fl}ay"));
                }
            }
            None => continue,
        }

        pig_latin.push_str(" ");
    }

    pig_latin
}
