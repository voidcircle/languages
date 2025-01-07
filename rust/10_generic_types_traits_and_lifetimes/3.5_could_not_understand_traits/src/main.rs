use could_not_understand_traits::{Amphibious, HoverCraft, LandCapable, Sedan, SUV};

fn main() {
    let new_sedan: Sedan = Sedan;
    road_trip(&new_sedan);

    let new_suv: SUV = SUV;
    road_trip(&new_suv);

    let new_hovercraft: HoverCraft = HoverCraft;
    traverse_frozen_lake(&new_hovercraft);
}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn traverse_frozen_lake(vehicle: &impl Amphibious) {
    vehicle.drive();
    vehicle.float();
}
