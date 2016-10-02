extern crate uom;

use uom::si::f32::*;
use uom::si::length::{yard};
//use uom::si::mass::{kilogram};
use uom::si::time::{minute};
use uom::si::velocity::{meter_per_second, foot_per_second};

fn main() {
    //let mass = Mass::new(50.0, kilogram);
    let length = Length::new(100.0, yard);
    let time = Time::new(1.0, minute);
    let velocity: Velocity = length / time;
    let more_length = length + length;
    //let error = length + time; // mismatched types [E0308]
    //let error: Velocity = mass / time; // mismatched types [E0308]

    println!("{:?} / {:?} = {:?}", length, time, velocity);
    println!("{length:?} + {length:?} = {more_length:?}", length = length, more_length = more_length);
    println!("{:?} m / s = {:?} f / s", velocity.get(meter_per_second),
        velocity.get(foot_per_second));
}
