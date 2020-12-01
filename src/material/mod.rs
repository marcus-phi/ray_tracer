use crate::hit::HitPoint;
use crate::math;
use crate::math::Ray;
use crate::math::Scatter;
use crate::texture::Texture;

pub trait Material {
    fn scatter(&self, r: Ray, hp: HitPoint) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
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
