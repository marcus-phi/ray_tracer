mod aabb;
mod color3;
mod ray;
mod scatter;
mod vec3;

pub use aabb::AABB;
pub use color3::Color3;
pub use ray::Ray;
pub use scatter::Scatter;
pub use vec3::Vec3;

use rand::prelude::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        if v.len() < 1.0 {
            return v;
        }
    }
}
