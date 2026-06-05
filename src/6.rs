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

    pub fn tick(&mut self) {
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
        turn(angle_error * 40.0); // 比例控制，誤差越大轉越快
        fire(0);
    }
}
