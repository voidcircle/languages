mod front_of_house;

pub use front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
