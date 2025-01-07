pub struct Sedan;

pub struct SUV;

pub struct HoverCraft;

impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan is driving hard");
    }
}

impl LandCapable for SUV {}

impl Amphibious for HoverCraft {}
impl LandCapable for HoverCraft {}
impl WaterCapable for HoverCraft {}

pub trait LandCapable {
    fn drive(&self) {
        println!("The car is driving");
    }
}

pub trait WaterCapable {
    fn float(&self) {
        println!("The vehicle is swimming..?");
    }
}
pub trait Amphibious: WaterCapable + LandCapable {}
