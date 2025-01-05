fn main() {
    let mut v: Vec<u8> = Vec::new();
    let _v2: Vec<u8> = vec![20, 100];

    v.push(4);
    // v.push(300); // no!
    v.push(20);

    let third: u8 = v[1]; // v[1000] will throw an error
                          // and &v[1] is llike I guess optional?
    println!("{third}");
    let third: &u8 = &v[1];
    println!("{third}");

    // v.push(5); // this causes a compiler error since the variable `third` does not expect the
    // original data to be changed. You might ask, why would the first element care about the
    // changes that happen at the end of the vector? That is involved with how Vector works. When
    // an element is added into a vector, the space where the new data comes into might be smaller
    // than the data. In this case, Rust just copies and pastes the whole data into other space,
    // which causes the references to other pre-exisitng data are allocating to the data that does
    // not exist anymore.

    println!("{third}");

    let third: Option<&u8> = v.get(0);

    if let Some(number) = third {
        println!("Somethign! {number}");
    } else {
        println!("NOthign!");
    }

    // match third {
    //     Some(number) => println!("Something! {number}"),
    //     _ => println!("Nothing!");,
    // }

    dbg!(v);

    let mut vs: Vec<String> = Vec::new();

    vs.push(String::from("Hello"));
    vs.push(String::from("World"));

    let third: String = vs[1].clone();
    println!("{third}");

    dbg!(vs);

    // let third: Option<&u8> = v.get(2);

    let mut vc: Vec<u32> = vec![200, 400, 100];

    for i in &vc {
        println!("{i}");
    }

    // the value &mut vc must have the reference to the data with mutabilty to be able to give the
    // variable i the each iterated data with the mutability of its own. And, we Dereferencing the
    // variable i.
    for i in &mut vc {
        *i += 50
    }

    for i in &vc {
        println!("{i}");
    }

    dbg!(vc);

    // as the variables that hold vectors go out of scope, it is dropped. And when the variable
    // itself is dropped, the contents are also dropped, too.
}
