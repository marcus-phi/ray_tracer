use crate::math::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction: direction.unit(),
        }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[test]
fn ray_new() {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0));
    assert_eq!(1.0, r.origin.x);
    assert_eq!(2.0, r.origin.y);
    assert_eq!(3.0, r.origin.z);
    assert_eq!(1.0, r.direction.len())
}

#[test]
fn ray_at_t() {
    let r = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 0.0, 0.0));

    let p = r.at(2.0);
    assert_eq!(3.0, p.x);
    assert_eq!(2.0, p.y);
    assert_eq!(3.0, p.z);
}
