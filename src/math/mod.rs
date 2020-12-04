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
        let v = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
        if v.len() < 1.0 {
            return v;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let v = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if v.len() < 1.0 {
            return v;
        }
    }
}

pub fn schlick_approx(cosine: f64, refract_index: f64) -> f64 {
    let r0 = (1.0 - refract_index) / (1.0 + refract_index);
    let r = r0 * r0;
    return r + (1.0 - r) * ((1.0 - cosine).powi(5));
}
