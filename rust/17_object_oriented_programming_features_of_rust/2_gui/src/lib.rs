pub trait Draw {
    fn draw(&self);
}

// Of course, we could have used the generic type `<T: Draw>` and add all sorts of generic type
// stuff for impl block. But what is going to happen is...
//
// By using that trait Draw, we can create multiple enums and structs. Anyway, we can impl Draw for
// {} to implement that trait. But, if we use the generic type, we will have to put the generic
// type on the variable which will strict us to use only ONE enum or ONE struct that implements the
// Draw trait. We cannot put multiple different structs or enums that do implement that trait; we
// can only put that identical one enum or struct that does implement the Draw trait. As a result,
// we won't be able to create this Screen with generic type like a struct that has a field of
// components that can contain multiple enums or structs that the only thing in common between is
// the trait Draw; we canNOT.
//
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         self.components
//             .iter()
//             .for_each(|current_component| current_component.draw());
//     }
// }
//
// However, if we do not use the generic type, but use Box<dyn Draw> then we can put the component
// any enums or/and structs that implement the Draw trait, which means we do not have to specific
// when defining the variable, and also put multiple of them.
//

pub struct Screen {
    // the components field will accept data of vector of Box<dyn Draw>
    // the reason why we have to add dyn keyword is because, anyway what we want to do is to have
    // a vector of any structs or enums that do implement the draw method defined in Draw trait.
    // That is the reason why we have to add the `dyn` syntax. However, since anything that
    // implements the draw method that is defined in Draw trait, we do not know the size of it.
    // That is why we have to add Box smart pointer since... literally we do not what the size is
    // going to be. By defining with Box smart pointer, we will know the size, which will be the
    // same size as a reference.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        self.components
            .iter()
            .for_each(|current_component: &Box<dyn Draw>| current_component.draw());
    }
}
