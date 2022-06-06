#![allow(unused_variables)]
mod vec;
use vec::*;

fn main() {
    let vec3_1 = Vec3::new(5., 10., 15.);
    let vec3_2 = Vec3::new(5., 5., 5.);

    let vec2_1 = Vec2::new(5., 10.);
    let vec2_2 = Vec2::new(5., 2.);

    println!("{}", vec3_1 * 5.);
}
