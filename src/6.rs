// Tutorial: Deflection
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function.
//
// Hint: p = p₀ + v₀t + ½at² (the third equation of kinematics)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    fn print_debug_info(&mut self) {
        debug!("distance: {}", target() - position());
        debug!(
            "distance.dot(): {}",
            (target() - position()).dot(target() - position())
        );
    }

    pub fn calculate_direction_to_target() -> Vec2<f64> {
        let direction = target() - position();
        let angle = direction.y.atan2(direction.x);
        vec2(angle.cos(), angle.sin())
    }

    pub fn tick(&mut self) {
        let to_target = angle_diff(heading(), target().angle());
        let distance = target() - position();
        let time_to_target = distance.length() / BULLET_SPEED;
        let target_after_t = (target() + target_velocity() * time_to_target).normalize();
        let to_target_after_t = angle_diff(heading(), target_after_t.angle());
        turn(to_target_after_t);
        accelerate(angle_diff(heading(), Self::calculate_direction_to_target()));
        fire(0);

        self.print_debug_info();
    }
}

// not done yet
