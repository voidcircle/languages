use std::error::Error;
use std::{
    fs::File,
    io::{self, Error as ioError, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("PANIC!");

    let _v: Vec<u8> = vec![1, 2, 3, 4, 5];

    // println!("{}", v[99]);

    let file: File = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error occured while creating a new file: {error:#?}"),
            },
            _ => panic!("Error occured while trying to open a hello.txt file: {error:#?}"),
        },
    };

    // If you do not want to use match expressions...
    //
    // File::open("hello.txt").unwrap_or_else(|open_err| {
    //     if open_err.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|create_err| {
    //             panic!("Error occured while creating a new file: {create_err:#?}");
    //         });
    //     }
    //     panic!("Error occured while trying to open a hello.txt file: {open_err:#?}");
    // });

    // If you do not want to use the `unwrape_or_lese` method and provides a good error message
    // when an error occures.
    // File::open("hello.txt").expect("hello.txt does not exist!")

    println!("{file:?}");

    File::open("asdjkfladfjkl.tst")?;

    Ok(())
}

fn read_file_to_string(file_name: String) -> Result<String, io::Error> {
    let selected_file_result: Result<File, ioError> = File::open(file_name);

    let mut selected_file = match selected_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut file_content: String = String::new();

    match selected_file.read_to_string(&mut file_content) {
        Ok(_) => return Ok(file_content),
        Err(err) => return Err(err),
    };
}

fn read_file_to_string_with_question_mark(file_name: String) -> Result<String, io::Error> {
    let mut file_content: String = String::new();
    // if you put a question mark,
    // if the method or whatever you put the question mark operator on returns Result<ERROR> then
    // it returns early on.
    // You can use it continuously on just one line.
    File::open(file_name)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}

fn get_last_char_of_first_line(text: &str) -> Option<char> {
    // of course, it works with Result<T, E> but it also works with Option<T>
    // It returns early if the value is None. If it is not, then the resultant value of the
    // expression would be Some().
    text.lines().next()?.chars().last()
}
