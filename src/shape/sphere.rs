use crate::hit::*;
use crate::material::*;
use crate::math::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat,
        }
    }

    fn get_hit_point(&self, r: Ray, t: f64) -> HitPoint {
        let p = r.at(t);
        HitPoint {
            t,
            p,
            n: (p - self.center) / self.radius,
            mat: &self.mat,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitPoint> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - 4.0 * a * c;

        if d > 0.0 {
            let t = (-b - d.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                return Some(self.get_hit_point(r, t));
            }
            let t = (-b + d.sqrt()) / (2.0 * a);
            if t < t_max && t > t_min {
                return Some(self.get_hit_point(r, t));
            }
        }
        None
    }

    fn bbox(&self) -> Option<AABB> {
        let r = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - r, self.center + r))
    }
}

#[test]
fn sphere_bbox() {
    let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Box::new(DummyMaterial {}));
    let bb = s.bbox().unwrap();

    assert_eq!(-1.0, bb.min.x);
    assert_eq!(-1.0, bb.min.y);
    assert_eq!(-1.0, bb.min.z);
    assert_eq!(1.0, bb.max.x);
    assert_eq!(1.0, bb.max.y);
    assert_eq!(1.0, bb.max.z);
}

#[test]
fn shere_hit() {
    let s = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Box::new(DummyMaterial {}));
    let r1 = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    let r2 = Ray::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
    let r3 = Ray::new(Vec3::new(10.0, 10.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));

    let p1 = s.hit(r1, f64::MIN, f64::MAX);
    let p2 = s.hit(r2, f64::MIN, f64::MAX);
    let p3 = s.hit(r3, f64::MIN, f64::MAX);

    match p1 {
        Some(p) => {
            assert_eq!(p.t, 8.0);
            assert_eq!(p.p.x, -2.0);
            assert_eq!(p.p.y, 0.0);
            assert_eq!(p.p.z, 0.0);
            assert_eq!(p.n.x, -1.0);
            assert_eq!(p.n.y, 0.0);
            assert_eq!(p.n.z, 0.0);
        }
        _ => assert!(false),
    }

    match p2 {
        Some(p) => {
            assert_eq!(p.t, 8.0);
            assert_eq!(p.p.x, 2.0);
            assert_eq!(p.p.y, 0.0);
            assert_eq!(p.p.z, 0.0);
            assert_eq!(p.n.x, 1.0);
            assert_eq!(p.n.y, 0.0);
            assert_eq!(p.n.z, 0.0);
        }
        _ => assert!(false),
    }

    match p3 {
        None => assert!(true),
        _ => assert!(false),
    }
}
