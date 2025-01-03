// On the first day of Christmas,
// my true love gave to me
// A partridge in a pear tree.
//
// On the second day of Christmas,
// my true love gave to me
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the third day of Christmas,
// my true love gave to me
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the fourth day of Christmas,
// my true love gave to me
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the fifth day of Christmas,
// my true love gave to me
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the sixth day of Christmas,
// my true love gave to me
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the seventh day of Christmas,
// my true love gave to me
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the eighth day of Christmas,
// my true love gave to me
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the ninth day of Christmas,
// my true love gave to me
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the tenth day of Christmas,
// my true love gave to me
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the eleventh day of Christmas,
// my true love gave to me
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
//
// On the twelfth day of Christmas,
// my true love gave to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree!

fn main() {
    const NUMBER_OF_MONTHS: usize = 12;
    const ORDINAL_NUMBERS_AND_THINGS: [[&str; 2]; NUMBER_OF_MONTHS] = [
        ["first", "And a partridge in a pear tree."],
        ["second", "Two turtle doves,"],
        ["third", "Three French hens,"],
        ["fourth", "Four calling birds,"],
        ["fifth", "Five golden rings,"],
        ["sixth", "Six geese a-laying,"],
        ["seventh", "Seven swans a-swimming,"],
        ["eighth", "Eight maids a-milking,"],
        ["ninth", "Nine ladies dancing,"],
        ["tenth", "Ten lords a-leaping,"],
        ["eleventh", "Eleven pipers piping,"],
        ["twelfth", "Twelve drummers drumming,"],
    ];
    const THE_FIRST_ELEMENT: &str = "A partridge in a pear tree.";
    const THE_LAST_FIRST_ELEMENT: &str = "And a partridge in a pear tree!";

    for current_index in 0..NUMBER_OF_MONTHS {
        println!(
            "On the {} day of Christmas,",
            ORDINAL_NUMBERS_AND_THINGS[current_index][0]
        );
        println!("my true love gave to me");

        for current_phrase_index in (0..current_index + 1).rev() {
            if current_index == 0 {
                println!("{THE_FIRST_ELEMENT}");
            } else if current_index == NUMBER_OF_MONTHS - 1 && current_phrase_index == 0 {
                println!("{THE_LAST_FIRST_ELEMENT}");
            } else {
                println!("{}", ORDINAL_NUMBERS_AND_THINGS[current_phrase_index][1]);
            }
        }

        if current_index != NUMBER_OF_MONTHS - 1 {
            println!("");
        }
    }
}
