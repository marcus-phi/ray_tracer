use crate::hit::HitPoint;
use crate::math::Ray;
use crate::math::Scatter;

pub trait Material {
    fn scatter(&self, r: Ray, hp: HitPoint) -> Option<Scatter>;
}

mod lambertian;
pub use lambertian::Lambertian;

mod metal;
pub use metal::Metal;

mod dielectric;
pub use dielectric::Dielectric;
