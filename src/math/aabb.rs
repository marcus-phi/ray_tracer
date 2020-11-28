use crate::math::Vec3;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(v1: Vec3, v2: Vec3) -> AABB {
        AABB {
            min: v1.min(v2),
            max: v1.max(v2),
        }
    }
}

impl ops::Add<AABB> for AABB {
    type Output = AABB;

    fn add(self, other: AABB) -> AABB {
        AABB::new(self.min.min(other.min), self.max.max(other.max))
    }
}

#[test]
fn aabb_new() {
    let bb = AABB::new(Vec3::new(1.0, 3.0, 5.0), Vec3::new(2.0, 2.5, 4.0));

    assert_eq!(1.0, bb.min.x);
    assert_eq!(2.5, bb.min.y);
    assert_eq!(4.0, bb.min.z);
    assert_eq!(2.0, bb.max.x);
    assert_eq!(3.0, bb.max.y);
    assert_eq!(5.0, bb.max.z);
}

#[test]
fn aabb_add_aabb() {
    let bb1 = AABB::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    let bb2 = AABB::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(3.0, 2.0, 1.0));
    let bb = bb1 + bb2;

    assert_eq!(-1.0, bb.min.x);
    assert_eq!(-2.0, bb.min.y);
    assert_eq!(-3.0, bb.min.z);
    assert_eq!(3.0, bb.max.x);
    assert_eq!(4.0, bb.max.y);
    assert_eq!(6.0, bb.max.z);
}
