use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(initial_value: T) -> MyBox<T> {
        MyBox::<T>(initial_value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x: u8 = 5;
    let y: &u8 = &x;
    let y2: Box<u8> = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5); // by dereferencing y, we have the access to use the data that the referecne
                       // is pointing at, which is 5 in this case.
    assert_eq!(*y2, 5); // the same thing happenes, but with box

    let y3: MyBox<u8> = MyBox::new(x);
    assert_eq!(*y3, 5); // the same as assert_eq!(*(y3.deref()), 5)

    let message: MyBox<String> = MyBox(String::from("Seol SO."));
    hello(&message); // it is working since Rust just calls the deref method automatically.
                     // &message = the reference to the MyBox, which is a smart pointer to the
                     // String, which is another smart pointer to the string data "Seol SO."
                     //
                     // Rust dereferences the message into
                     // message = MyBox<String>
                     //
                     // Rust dereferences the MyBox<String> into (And this is possible because we
                     // implemented the Deref trait over there.)
                     // MyBox<String> = String
                     //
                     // Rust dereferences the String into
                     // String = &str
                     //
                     // If Rust did not dereference for us, then we would have had to write the
                     // smae like
                     //
                     // `hello(&(*message)[..])`
                     //
                     // this.
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
