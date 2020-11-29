use crate::math::Color3;
use crate::math::Ray;

#[derive(Debug)]
pub struct Scatter {
    pub attenuation: Color3,
    pub scattered_ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Color3, scattered_ray: Ray) -> Scatter {
        Scatter {
            attenuation,
            scattered_ray,
        }
    }
}
