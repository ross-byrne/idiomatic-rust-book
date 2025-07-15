#[derive(Debug)]
pub struct Bicycle {
    make: String,
    model: String,
    size: i32,
    colour: String,
}

#[derive(Debug)]
pub struct BicycleBuilder {
    bicycle: Bicycle,
}

impl BicycleBuilder {
    pub fn new() -> Self {
        Self {
            bicycle: Bicycle {
                make: String::new(),
                model: String::new(),
                size: 0,
                colour: String::new(),
            },
        }
    }

    pub fn build(self) -> Bicycle {
        self.bicycle
    }
}
