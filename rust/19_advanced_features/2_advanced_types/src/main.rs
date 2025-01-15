type Thunk = Box<dyn Fn() + Send + 'static>;
// better not to use import stuff???
type Result<T> = std::result::Result<T, std::io::Error>;

fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 10;

    println!("{}", x + y);

    let _f: Thunk = Box::new(|| println!("hi"));

    let something1: u32 = loop {
        let something2: Option<bool> = Some(false);
        // in this case, continue returns ! value, which means it returns NO value. The continue
        // keyword does not really return any value but goes to the top and repeat the same code,
        // and Rust knows that this situations might happen and just define the some_value value as
        // u32 even Rust sees one of the arms of the match expression returning a continue, which
        // is ! value. (all arms of match expression must return the same type value.)
        let some_number: u32 = match something2 {
            Some(inner_value) => {
                if inner_value {
                    20
                } else {
                    10
                }
            }
            None => continue,
        };

        break some_number;
    };

    // also, ! value is used for panic! macro.

    let something3: bool = false;
    let something4: bool = match something3 {
        true => false,
        // in this case, it panics, returning... technically a type of value that can never have
        // any value. So, this expression returns nothing (! value)
        //
        // ! type is also called Never Type.
        // `This expression returns never.`
        false => panic!("OHHHH! Somethign3 is FALSE!!!!!"),
    };

    // println!("forever");
    //
    // // this loop returns ! since it never ends.
    // // as you might have guessed, it will become an expression that does not return never(!) as
    // // soon as it includes break keyword since the break keyword forces the loop to return
    // // something. (wait.. but we can break a loop without returning any value...)
    // loop {
    //     println!("ever...");
    // }

    // Dynamically sized types
    // Sometimes, Rust does not know how much space certain types would take up on the memory. As
    // an example, str. str is a string slice whose size is known only at the runtime. So, when we
    // define a variable that holds a string slice, we are NOT using just str since str is just the
    // type whose is unknown. But instead, we use &str, which is a reference to the string slice
    // with an extra space for the length of it. Since it is just that Rust needs to know the size
    // of certain data to store, we can use Box<str> or Rc<str> or any smart pointers. But we just
    // choose to use the reference, whose size is KNOWN.
    //
    // Another example of dsts is the traits. All traits defined on rust files are treated as
    // a dsts. So, that is why we have to use like Box<dyn SomeTrait> or something like this.
    //
    // Another example of dsts is the generics. See more detailed below.
}

fn _takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {}
fn _returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    Box::new(|| {})
}

fn _takes_short_type(_f: Thunk) {}
fn _returns_short_type() -> Thunk {
    Box::new(|| {})
}

// since ! type is a type, we can add this to a function
fn _returns_never_value() -> ! {
    loop {
        println!("Hello!");
    }
}

fn _generic<T>(_t: T) {}
// _generic_1 function is exactly same as _generic function and this function(generic_1) is how
// Rust would have treated the function above. It adds Sized
fn _generic_1<T: Sized>(_t: T) {}
// When we do NOT know the size of generic type T, we can add the ? operator at the front. Of
// course, we have to add the reeference of T. &T or Box<T> or Rc<T> or any other smart pointers.
fn _generic_2<T: ?Sized>(_t: Box<T>) {}
