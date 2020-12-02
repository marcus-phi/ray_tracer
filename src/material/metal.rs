use crate::hit::HitPoint;
use crate::material::Material;
use crate::math;
use crate::math::Color3;
use crate::math::Ray;
use crate::math::Scatter;

pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color3, fuzz: f64) -> Box<Metal> {
        Box::new(Metal { albedo, fuzz })
    }
}

impl Material for Metal {
    fn scatter(&self, r: Ray, hp: HitPoint) -> Option<Scatter> {
        let reflected = r.direction.reflect(hp.n);
        let scattered_ray = Ray::new(hp.p, reflected + self.fuzz * math::random_in_unit_sphere());

        if scattered_ray.direction.dot(hp.n) > 0.0 {
            Some(Scatter::new(self.albedo, scattered_ray))
        } else {
            None
        }
    }
}
