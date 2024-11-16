// Tutorial: Guns
// Destroy the asteroid.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    // Uncomment me, then press Ctrl-Enter (Cmd-Enter on Mac) to upload the code.
    pub fn tick(&mut self) {
        accelerate(vec2(100.0, 0.0));
        fire(0);
    }
}

// this 2.033s
// best 1.717s
