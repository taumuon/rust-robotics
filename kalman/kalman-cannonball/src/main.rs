extern crate nalgebra;
extern crate rand;

use nalgebra::*;
use rand::*;
use std::f64;

// http://greg.czerniak.info/guides/kalman1/
// http://www.bzarg.com/p/how-a-kalman-filter-works-in-pictures/
fn main() {

    let angle = 45.0;
    let muzzle_velocity = 100.0;
    let gravity = Vector2::new(0.0, -9.81); // [0,-9.81];
    let initial_velocity_x = muzzle_velocity * (angle * f64::consts::PI/180.0).cos();
    let initial_velocity_y = muzzle_velocity * (angle * f64::consts::PI/180.0).sin();
    let initial_velocity = Vector2::new(initial_velocity_x, initial_velocity_y);
    let initial_location = Vector2::new(0.0, 0.0);
    let initial_acceleration = Vector2::new(0.0, 0.0);

    let delta_time = 0.1;

    let mut location = initial_location;
    let mut velocity = initial_velocity;
    let mut acceleration = initial_acceleration;

    for x in 0..140 {
        acceleration = gravity;
        velocity += acceleration * delta_time;
        location += velocity * delta_time;
        println!("{} {} {}", x, location[0], location[1]);
    }
}
