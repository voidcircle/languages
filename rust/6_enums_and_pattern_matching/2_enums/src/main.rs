// This way is bad
// #[derive(Debug)]
// enum Messages {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColour(u8, u8, u8),
// }

// this way is good

enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(u8, u8, u8),
}

impl Messages {
    fn send() -> bool {
        return true;
    }
    fn edit(&self, to: String) -> bool {
        return true;
    }
    fn call(&self) -> bool {
        return true;
    }
}

fn main() {
    let message = Messages::Move { x: 20, y: 400 };

    message.call();
    message.edit(String::from("Hello?")); // this would not make any senses tho

    Messages::send();
    Messages::Write(String::from("Hello World"));
}
