// Tutorial: Acceleration
// Fly through the target circle.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        accelerate(vec2(100.0, 0.0));
    }
}

// this 3.883s
// best 2.383s
