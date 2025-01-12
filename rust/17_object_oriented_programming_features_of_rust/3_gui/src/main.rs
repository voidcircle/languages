use gui::{Button, Draw, Screen};

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("AHHH!! A select box is being drawn!!");
    }
}

fn main() {
    let screen: Screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 300,
                height: 500,
                options: vec![String::from("A"), String::from("B"), String::from("C")],
            }),
            Box::new(Button {
                width: 20,
                height: 10,
                label: String::from("Submit"),
            }),
        ],
    };

    screen.run();
}
