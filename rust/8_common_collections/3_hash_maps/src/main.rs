use std::{collections::HashMap, u8};

fn main() {
    let from_something = get_something();

    println!("{from_something}");

    let some_string: String = String::from("Hello world it is a wonderful world is not it ?");
    let word_counter_for_some_string: HashMap<String, u8> = count_words(some_string);

    dbg!(word_counter_for_some_string);

    // if let Some(actual_score) = from_something {
    //     println!("{actual_score}");
    // } else {
    //     println!("Notghin!");
    // }
}

fn get_something() -> u8 {
    let mut scores: HashMap<String, u8> = HashMap::new();

    let team_blue: String = String::from("Blue");

    // scores.insert basically takes the ownership. You should make a clone of it and puts the
    // cloned one.
    scores.insert(team_blue.clone(), 10);
    scores.insert(String::from("Yellow"), 50);

    // just overwrites it.
    // The older data will be just ignored.
    scores.insert(team_blue.clone(), 20);

    // check if the key "Yellow" exists, if it does NOT then insert 40 for that key.
    scores.entry(String::from("Yellow")).or_insert(40);
    scores.entry(String::from("Red")).or_insert(100);

    println!("{team_blue}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let searching_team: String = String::from("Yellow");
    let something: u8 = scores.get(&searching_team).copied().unwrap_or(0);

    // match something {
    //     Some(score) => println!("{searching_team}'s score is: {score}."),
    //     None => println!("{searching_team} cannot be found."),
    // }

    something
}

fn count_words(word: String) -> HashMap<String, u8> {
    let mut word_counter: HashMap<String, u8> = HashMap::new();

    for current_word in word.split_whitespace() {
        let count = word_counter.entry(String::from(current_word)).or_insert(0);
        *count += 1;

        // Or you can do the same thing in this way too
        //
        // word_counter
        //     .entry(String::from(current_word))
        //     .and_modify(|prev_word_count| *prev_word_count += 1)
        //     .or_insert(1);
    }

    word_counter
}
