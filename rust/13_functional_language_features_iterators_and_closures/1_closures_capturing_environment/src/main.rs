use std::thread;

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, preferred_shirt_colour: Option<ShirtColour>) -> ShirtColour {
        preferred_shirt_colour.unwrap_or_else(|| self.get_most_stocked())
    }

    fn get_most_stocked(&self) -> ShirtColour {
        let mut red_num: usize = 0;
        let mut blue_num: usize = 0;

        for current_shirt in &self.shirts {
            match current_shirt {
                ShirtColour::Red => red_num += 1,
                ShirtColour::Blue => blue_num += 1,
            }
        }

        if red_num >= blue_num {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u16| -> u16 {
        println!("We got an num: {num}");
        num
    };

    println!("{}", expensive_closure(2));

    // these are really good examples of showing the differences between functions and closures and
    // even between closures.
    //
    // However, in the case of the closures without any type annotations, you must use the closure
    // for one types. What I mean is that you cannot like use the function with the parameter with
    // the i32 type and use it with u8 type later since the compiler will assume that the closure
    // has to have the data type of i32, which is the first use case.

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1;

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    //////

    let mut list = vec![1, 2, 3];

    println!("Before defining closure: {list:?}"); // this line cannot be moved to right before the
                                                   // `borrows_mutably` function since the list
                                                   // varaible in the println statement is like let
                                                   // something = &list and the `borrows_mutably`'s list is like let something = &mut list. As you know there CANNOT be a mutable reference and immutable reference at a time since the immutable reference do not expect the original data to be modified.

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    println!("Before defining closure: {list:?}");

    // even though the body of this closure is a immutable reference, Rust reuqires the list
    // variable to take the ownership of it and move it to the new thread since the new thread can
    // live longer than the main thread, which is in this case, list will be invalid in the new
    // thread. So, it reuiqres you to put the move keyword to move the list's ownership to the new
    // thread. If you try to print the list variable after this, you will get an error as if the
    // list variable has been moved, which is actually what happened.
    thread::spawn(move || println!("From new thread: {list:?}"))
        .join()
        .unwrap();

    let something1234: Option<i32> = None;

    // println!("{list:?}"); // error! it has been moved.

    let mut rects: Vec<Rectangle> = Vec::from([
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
        Rectangle {
            width: 10,
            height: 1,
        },
    ]);

    let mut sort_operations: Vec<String> = Vec::new();
    let value = String::from("hahahahahah");

    let mut sort_operations_in_num: u8 = 0;
    // does not take any variables from the environment, which means it can be called multiple
    // times.
    //
    // it does not capture, mutate, or move.
    rects.sort_by_key(|r| {
        sort_operations_in_num += 1;
        r.width
    });
    rects.sort_by_key(|r| {
        sort_operations.push(value.clone()); // if we do not clone it, this closure will be the
                                             // closure that can be called only once. (FnMut)
                                             //
                                             // As soon as we add the clone method to it, it will
                                             // no loger mutate anything since it just cloning
                                             // things.
        r.width
    });

    println!("{rects:#?}");
    println!("{sort_operations:#?}");
    println!("{sort_operations_in_num}");
}
