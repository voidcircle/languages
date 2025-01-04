struct User {
    name: String, // why String? Not &str?
    username: String,
    email: String,
    age: u8,
    active: bool,
    followings_num: u16,
    followers_num: u16,
}

struct Colour(u8, u8, u8);
struct Point(u8, u8, u8);

fn main() {
    let user1 = User {
        name: String::from("Seol SO"),
        username: String::from("seolso"),
        email: String::from("voidcircle080111@gmail.com"),
        age: 200,
        active: false,
        followings_num: 300,
        followers_num: 20,
    };

    user1.followings_num;

    let mut user2 = User {
        name: String::from("Seol SO"),
        username: String::from("seolso"),
        email: String::from("voidcircle080111@gmail.com"),
        age: 200,
        active: false,
        followings_num: 300,
        followers_num: 20,
    };

    user2.email = String::from("hi@voidcircle.xyz");

    let user3 = build_user(
        String::from("username1"),
        String::from("something@example.com"),
    );

    let user4 = User {
        email: String::from("Hel"),
        ..user3
    };

    // println!("{}", user3.username); // impossibel because when user4 get inherited from user3,
    // user3 just gives everything to user4 so the fields in user3 with complex data type(for
    // exmaple email, username) would have just moved away to user4. However, fields in user3 with
    // simple data type for example age or active would have been copied and the copied one would
    // have been what has been moved away so it is valid for some fields in user3 but we cannot ues
    // user3 as a whole since it has some wholes.

    // thse two tuples below have DIFFERENT TYPES since they are the instances of different sturcts.
    let black_colour = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    black_colour.1; // 0
}

fn build_user(username: String, email: String) -> User {
    User {
        name: String::from("Something"),
        username, // shorthand!
        email,
        age: 200,
        active: false,
        followings_num: 200,
        followers_num: 20,
    }
}
