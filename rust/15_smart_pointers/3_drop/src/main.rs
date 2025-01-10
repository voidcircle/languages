struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    drop(c); // we cannot call the `drop` method like `c.drop()` since Rust would do it
             // automatically when c goes out of scope, which causes a double free error that the
             // same data is being freed twice.
             //
             // We can do it still in the way it is written, but... for most cases, we do not have
             // to.

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // if c was NOT droppped,
    // variables are dropped in reverse since they would be stored in the stack.
    // So, C is created, and D is created.
    // So, D is dropped, and C is dropped.
}
