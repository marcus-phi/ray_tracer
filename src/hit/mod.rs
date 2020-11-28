use crate::math::Ray;
use crate::math::Vec3;
use crate::math::AABB;

#[derive(Debug)]
pub struct HitPoint {
    pub t: f64,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitPoint>;
    fn bbox(&self) -> AABB;
}
