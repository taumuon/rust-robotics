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
    let g = -9.81;
    let gravity = Vector2::new(0.0, g);
    let initial_velocity_x = muzzle_velocity * (angle * f64::consts::PI/180.0).cos();
    let initial_velocity_y = muzzle_velocity * (angle * f64::consts::PI/180.0).sin();
    let initial_velocity = Vector2::new(initial_velocity_x, initial_velocity_y);
    let initial_location = Vector2::new(0.0, 0.0);
    let initial_acceleration = Vector2::new(0.0, 0.0);

    let delta_time = 0.1;

    let mut location = initial_location;
    let mut velocity = initial_velocity;
    let mut acceleration = initial_acceleration;

    let A = Matrix4::new(0.0, 1.0, 0.0, 0.0,
                         0.0, 0.0, 0.0, 0.0,
                         0.0, 0.0, 0.0, 1.0,
                         0.0, 0.0, 0.0, 0.0);

    // TODO: really want B to be a a 4x1 matrix, and u to be a scalar instead of a 4x1 vector
    //  - not present in nalgebra ?!
    let B = Matrix4::new(0.0, 0.0, 0.0, 0.0,
                         0.0, 0.0, 0.0, 0.0,
                         0.0, 0.0, 0.0, 0.0,
                         0.0, 0.0, 0.0, 1.0);

    let u = Vector4::new(0.0, 0.0, 0.0, g);

    let mut state = Vector4::new(initial_location[0], initial_velocity[0], initial_location[1], initial_velocity[1]);

    for x in 0..145 {
        // acceleration = gravity;
        // velocity += acceleration * delta_time;
        // location += velocity * delta_time;
        // let x_loc = location[0];
        // let y_loc = location[1]; 
        let state_dot = (A * state) + (B * u);
        if (x == 0)
        {
            println!("{}", state);
            println!("{}", state_dot);
        }
        state += (state_dot * delta_time);
        if (x == 0)
        {
            println!("{}", state);
        }

        let x_loc = state[0];
        let y_loc = state[2];

        println!("{} {} {}", x, x_loc, y_loc);
    }
}
