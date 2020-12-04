use crate::hit::HitPoint;
use crate::material::Material;
use crate::math;
use crate::math::Color3;
use crate::math::Ray;
use crate::math::Scatter;
use rand::prelude::*;

pub struct Dielectric {
    refract_index: f64,
}

impl Dielectric {
    pub fn new(refract_index: f64) -> Box<Dielectric> {
        Box::new(Dielectric { refract_index })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: Ray, hp: HitPoint) -> Option<Scatter> {
        let cosine = r.direction.dot(hp.n.unit());

        let outward_normal = if cosine > 0.0 { hp.n * -1.0 } else { hp.n };
        let ni_over_nt = if cosine > 0.0 {
            self.refract_index
        } else {
            1.0 / self.refract_index
        };

        let refracted = r.direction.refract(outward_normal, ni_over_nt);
        let reflect_prob = if refracted.is_some() {
            math::schlick_approx(cosine.abs(), self.refract_index)
        } else {
            1.0
        };

        let mut rng = rand::thread_rng();
        let scattered_ray = if rng.gen::<f64>() < reflect_prob {
            let reflected = r.direction.reflect(hp.n);
            Ray::new(hp.p, reflected)
        } else {
            Ray::new(hp.p, refracted.unwrap())
        };

        Some(Scatter::new(Color3::new(1.0, 1.0, 1.0), scattered_ray))
    }
}
