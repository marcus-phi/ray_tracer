use crate::hit::HitPoint;
use crate::math;
use crate::math::Ray;
use crate::math::Scatter;
use crate::texture::Texture;

pub trait Material {
    fn scatter(&self, r: Ray, hp: HitPoint) -> Option<Scatter>;
}

pub struct Lambertian<'a> {
    pub albedo: &'a dyn Texture,
}

impl<'a> Lambertian<'a> {
    pub fn new(albedo: &'a dyn Texture) -> Lambertian<'a> {
        Lambertian { albedo }
    }
}

impl<'a> Material for Lambertian<'a> {
    fn scatter(&self, _: Ray, hp: HitPoint) -> Option<Scatter> {
        let target = hp.p + hp.n + math::random_in_unit_sphere();
        Some(Scatter::new(
            self.albedo.value(0.0, 0.0),
            Ray::new(hp.p, target - hp.p),
        ))
    }
}

pub struct DummyMaterial {}

impl Material for DummyMaterial {
    fn scatter(&self, _: Ray, _: HitPoint) -> Option<Scatter> {
        None
    }
}
