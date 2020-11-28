use crate::math::Vec3;
use crate::math::AABB;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn bbox(&self) -> AABB {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        AABB::new(self.center - r, self.center + r)
    }
}

#[test]
fn sphere_bbox() {
    let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let bb = s.bbox();

    assert_eq!(-1.0, bb.min.x);
    assert_eq!(-1.0, bb.min.y);
    assert_eq!(-1.0, bb.min.z);
    assert_eq!(1.0, bb.max.x);
    assert_eq!(1.0, bb.max.y);
    assert_eq!(1.0, bb.max.z);
}
