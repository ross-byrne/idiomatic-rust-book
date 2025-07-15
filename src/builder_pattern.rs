#[derive(Debug)]
#[allow(dead_code)]
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

// Implements builder pattern as a fluent builder
// i.e. allows "method" chaining by returning the builder from each function
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

    pub fn with_make(self, make: &str) -> Self {
        Self {
            bicycle: Bicycle {
                make: make.into(),
                ..self.bicycle
            },
        }
    }

    pub fn with_model(self, model: &str) -> Self {
        Self {
            bicycle: Bicycle {
                model: model.into(),
                ..self.bicycle
            },
        }
    }

    pub fn with_size(self, size: i32) -> Self {
        Self {
            bicycle: Bicycle {
                size,
                ..self.bicycle
            },
        }
    }

    pub fn with_colour(self, colour: &str) -> Self {
        Self {
            bicycle: Bicycle {
                colour: colour.into(),
                ..self.bicycle
            },
        }
    }
}

pub fn builder_example() {
    let bike = BicycleBuilder::new()
        .with_make("cheese")
        .with_model("sliver")
        .with_size(50)
        .with_colour("red")
        .build();

    println!("bike: {:#?}", bike);
}
