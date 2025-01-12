pub trait Draw {
    fn draw(&self) {}
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("AHHH!! A button is being drawn!!");
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components
            .iter()
            .for_each(|current_component: &Box<dyn Draw>| current_component.draw());
    }
}
