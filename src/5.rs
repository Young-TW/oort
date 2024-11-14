// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)
//
// p.s. You can change your username by clicking on it at the top of the page.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    fn print_debug_info(&mut self, dp: Vec2, time_to_target: f64, direction: Vec2, faster_angle: f64) {
        debug!("distance to target: {}", dp.length());
        debug!("time to target: {}", time_to_target);
        debug!("direction: {}", direction);
        debug!("faster angle: {}", faster_angle);
        debug!("current tick: {}", current_tick());
    }

    pub fn tick(&mut self) {
        let distance = target() - position();
        let time_to_target = distance.length() / BULLET_SPEED;

        let target_after_t = target() + target_velocity() * time_to_target;
        let distance_after_t = distance + target_velocity() * time_to_target;
        let time_to_target_after_t: f64 = distance_after_t.length() / BULLET_SPEED;

        let direction = distance_after_t + target_velocity() * time_to_target_after_t;

        turn(angle_diff(heading(), direction.angle()));

        Self::print_debug_info(self, distance, time_to_target, direction, angle_diff(direction.angle(), target().angle()));
        draw_line(position(), target(), 0x00ff00);
        draw_line(position(), direction, 0xffffff);
        fire(0);
    }
}
