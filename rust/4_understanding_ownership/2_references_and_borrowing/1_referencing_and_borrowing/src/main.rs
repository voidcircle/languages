fn main() {
    // 1. s1 is created
    // 2. `calculate_length` is called with the s1 referenced(which means the ownership does not
    //    leave!)
    // 3. the function returns the length of the s1 but still s1 will be valid even after that
    //    function is called because the argument passed down to that function is just referencing
    //    to the variable NOT moved out.

    let s1: String = String::from("Hello");

    let len1: usize = calculate_length(&s1);

    println!("String \"{s1}\"'s length is {len1}");

    /////////////////////////////////////////

    // 1. s2 is created.
    // 2. `get_changed_string` is called with s2 mut moved out. the same as the previous one, s2
    //    will still be valid since s2 is passed down to the function with only the referencing
    //    data only. However with `&mut` keyword, it can know edit the data directly.
    // 3. len2 is created with the length of the CHANGED s2.
    // 4. since the data on the heap that is holded by s2 has been changed, when the print
    //    statement is called, s2 will be the chnaged one.

    let mut s2 = String::from("   hello, ");

    let len2: usize = get_changed_string(&mut s2);

    println!("String \"{s2}\"'s length is {len2}");

    /////////////////////////////////////////

    // 1. mutable s3 is defined with the string
    // 2. s4 and s5 are created. But the problem is that we cannot leave two or more variables
    //    borrowing the same variables. With just them being defined like that would be fine but as
    //    soon as they are introduced to the code and used(for example, the print statement uses
    //    those variables), the compiler will throw an error because we CANNOT have two variables
    //    referencing to the same data at the same time.

    // There cannot be any borrowing mutability more than once at a time
    //
    // let mut s3 = String::from("Hello, World!");
    //
    // let s4 = &mut s3;
    // let s5 = &mut s3;
    //
    // println!("{s4}, {s5}");

    /////////////////////////////////////////

    // In this, it is a big different.
    // 1. mutable s3 is created
    // 2. a new scope is created.
    // 3. inside of the new scope, r1 is created with the mutability of s3(borrowing)
    // 4. prints r1
    // 5. r1 goes out of scope; r1 is removed(freed!)
    // 6. r2 is created in the same way that r1 was created. !!! It is POSSIBLE because it is NOT
    //    that two variables referencing the data with the mutability at the same time. They are in
    //    different scopes; so it is possible.
    // 7. prints r2.

    let mut s3 = String::from("Hello");

    {
        let r1 = &mut s3;
        println!("{r1}");
    } // r1 goes out of scope.

    let r2 = &mut s3; // possibel because there is still ONE mutable borrowing happening at a time.

    println!("{r2}");

    /////////////////////////////////////////

    // 1. mutable s4 is created
    // 2. r1 to r4 is referencing to the same s4 withOUT the mutability
    // 3. r5 is referencing to the s4 with the mutability
    // 4. but they cannot be printed or something because as soon as one of those variables are
    //    introduced to the code and used, _r5 will be angry because r5 is the only one that can
    //    edit the data while other variables are referencing to the same data withOUT the
    //    multabiltiy. r1 to r4 are afraid of the original data being edited or changed.

    let mut s4 = String::from("hello");

    let _r1 = &s4; // no problem because it is just referencing to the data that is hold by s4
    let _r2 = &s4;
    let _r3 = &s4;
    let _r4 = &s4;
    let _r5 = &mut s4; // big problem as soon as there is a variable that is borrowing the
                       // mutability too because other variables that are referencing to the same
                       // variable do not expect the original data to be changed.
                       //
                       // Being with the print statement commented is okay but as soon as the print
                       // statements are NOT commented, then LSP will throw an error to you.

    // println!("{_r1}, {_r2}, {_r3}");
    println!("{_r5}"); // it IS possible because r5 is the only one used.

    /////////////////////////////////////////

    // 1. mutable s5 is created
    // 2. r1 and r2 are created referencing to the s5 variable without any problem
    // 3. prints r1 and r2
    // 4. r3 is created with the mutability
    // 5. prints r3
    //
    //
    // This IS possible because the situation where does not allow multiple variables referencing
    // the same data and some of them with mutability and some of them without mutability is when
    // they are in the same scope.
    //
    // As soon as r1 and r2 are printed, the compiler knows that they will never be used again. So,
    // they will be written as gone! And r3 is createed and used with the mutability of s5, which
    // is possible since r3 would be the only thing that referencing the data with mutability.

    let mut s5 = String::from("hello");

    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s5; // no problem
    println!("{r3}");

    // println!("{r1}"); // will throw an error because the scopes of r1 and r2 will end when the
    // first r1 and r2's print, but this commented print statement runs, they will be called back
    // again, which makes the r1 and r2 nervous because they are afraid of the original data being
    // changed!
}

fn calculate_length(s: &String) -> usize {
    // s.push_str("Hello"); // impossible becasue `s` does not have the ownership over the actual
    // data content on the heap. It is immutable like all variables are by default.
    s.len()
}

fn get_changed_string(change_something: &mut String) -> usize {
    change_something.push_str("World!!!!     ");
    change_something.len()
}
