mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            // take_order is NOT valid since add_to_waitlist is a child of front_of_house and
            // hosting so it can access to its parent. But since take_order function is a child of
            // serving module, it has to be public to make this function work.
            // crate::front_of_house::serving::take_order();
        }

        fn sea_at_tale() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        // toast should be marked with the `pub` keyword, because the struct will be public because
        // of the keyword `pub` before the `struct` keyword, but its fields will be private stil.
        pub toast: String,
        seasonal_fruit: String,
        appetizer: Appetizer,
    }

    impl Breakfast {
        // since we put the seasonal_fruit as a private field, we HAVE make an associated function
        // that defines for us.
        pub fn summer(toast: &str, appetizer: Appetizer) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
                appetizer,
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();

        super::deliever_order();
    }

    fn cook_order() {}
}

fn deliever_order() {}

pub fn eat_at_restaurant() {
    // those two examples would not be working since the modules and functions are not PUBLIC yet.

    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye", back_of_house::Appetizer::Soup);
    meal.toast = String::from("wheet");

    // we cannot define it like this because the seasonal_fruit is a private field but we cannot do
    // anything with it! So, we have to call the associated function taht defines itself for us
    // like the `summer` method
    // let mut meale = back_of_house::Breakfast{
    //     toast: String::from("Rye"),
    //     seasonal
    // }

    println!("I would like {} toast please", meal.toast);

    // we cannot access to the seasonal fruit. since it was deinfed without the keyword `pub`
    // meal.seasonal_fruit
}
