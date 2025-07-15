use builder_pattern::{Bicycle, BicycleBuilder};

mod builder_pattern;

fn main() {
    let bike_builder = BicycleBuilder::new();

    println!("builder: {:#?}", bike_builder);
}
