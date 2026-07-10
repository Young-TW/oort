// Tutorial 4: Rotation (solution)
// Destroy the asteroid. The target is in a random
// location given by the "target()" function.
use oort_api::prelude::*;

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        let heading_error = angle_diff(heading(), (target() - position()).angle());
        turn(heading_error);
        fire(0);
    }
}
