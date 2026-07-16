// Tutorial: Radar
// Destroy the enemy ships. Use your radar to find them.
// Hint: Press 'g' in-game to show where your radar is looking.
// Hint: Press 'n' to single-step.
// Hint: Use the set_radar_heading() function to keep your radar pointed at a
// target, or to search for a new one.
//
// Join the Discord at https://discord.gg/vYyu9EhkKH for Oort discussion and
// tournament results.
use oort_api::prelude::*;

const BULLET_SPEED: f64 = 1000.0; // m/s

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    pub fn tick(&mut self) {
        set_radar_heading(radar_heading() + radar_width());
        if let Some(contact) = scan() {
            accelerate(0.1 * (contact.position - position()));
        }

        let distance = target() - position();
        let time_to_target = distance.length() / BULLET_SPEED;

        let distance = target() - position();
        let mut t = distance.length() / BULLET_SPEED;
        for _ in 0..5 {
            let predicted = target() + target_velocity() * t - position();
            t = predicted.length() / BULLET_SPEED;
        }
        let aim = target() + target_velocity() * t;
        let aim_direction = aim - position();
        let angle_error = angle_diff(heading(), aim_direction.angle());
        turn(angle_error * 90.0); // 比例控制，誤差越大轉越快
        fire(0);
    }
}
