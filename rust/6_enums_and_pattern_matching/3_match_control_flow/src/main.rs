#[derive(Debug)]
enum USStates {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USStates),
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(USStates::Iowa);
    let coin3 = Coin::Quarter(USStates::Arizona);
    let coin4 = Coin::Quarter(USStates::NorthDakota);

    println!("{}", get_value_in_cents(coin1));
    println!("{}", get_value_in_cents(coin2));
    println!("{}", get_value_in_cents(coin3));
    println!("{}", get_value_in_cents(coin4));
}

fn get_value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_states) => {
            match us_states {
                USStates::Alaska => {
                    println!("AAAA!");
                }
                USStates::Iowa => {
                    println!("IIII!");
                }
                _ => {}
            }

            println!("created in {us_states:?}");
            25
        }
    }
}
