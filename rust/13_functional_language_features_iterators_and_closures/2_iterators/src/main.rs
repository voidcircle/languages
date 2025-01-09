#[derive(Debug)]
struct Shoe {
    size: u8,
    style: String,
}

fn get_shoes_in_size(shoes: Vec<Shoe>, size: u8) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|current_shoe| current_shoe.size == size)
        .collect()
}

fn main() {
    let v1: Vec<u8> = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();

    // we did not have to make the v1_iter mutable because for loop takes the onwership of it and
    // does everything that we expect it to do behind the senses.
    //
    // for current_v in v1_iter {
    //     println!("{current_v}");
    // }

    // in the case of this, we have to make the v1_iter mutable since every time we call `next`
    // method, it changes the state that counts how many inside of the trait.
    //
    //
    //
    // IMPORTANT
    //
    // the `next` method gives you a immutable reference.
    // call `into_iter` method when defining v1_iter variable if you want to have an owned value,
    // call `into_mut` method when defining v1_iter variable if you want to have a mutable
    // reference to the value.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&5));
    assert_eq!(v1_iter.next(), None);

    // it is going to print 0 since the count become the same as the len of the iter.
    println!("{}", v1_iter.sum::<u8>());

    // let's make another one.
    let v2_iter = v1.iter();
    println!("{}", v2_iter.sum::<u8>());
    // then we can NOT use v2_iter anymore because sum method takes the ownership...

    let v3_iter = v1.iter();
    // map method returns an iter
    for current_v in v3_iter.map(|x| x + 1) {
        println!("Got {current_v}");
    }

    let v4_iter = v1.iter();
    let v4_iter_increased_by_one: Vec<u8> = v4_iter.map(|x| x + 1).collect();
    println!("{v4_iter_increased_by_one:#?}");

    let v5_iter = v1.into_iter();
    let v5_iter_without_even_numbers: Vec<u8> = v5_iter.filter(|x| *x % 2 != 0).collect();
    println!("{v5_iter_without_even_numbers:#?}");

    let shoes: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let size: u8 = 10;

    println!("Size: {size}");
    println!("Shoes: {:#?}", get_shoes_in_size(shoes, size));
}
