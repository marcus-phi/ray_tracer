use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }

    pub fn min(self, v: Vec3) -> Vec3 {
        Vec3::new(
            f64::min(self.x, v.x),
            f64::min(self.y, v.y),
            f64::min(self.z, v.z),
        )
    }

    pub fn max(self, v: Vec3) -> Vec3 {
        Vec3::new(
            f64::max(self.x, v.x),
            f64::max(self.y, v.y),
            f64::max(self.z, v.z),
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f64) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f64) -> Vec3 {
        Vec3::new(self.x / s, self.y / s, self.z / s)
    }
}

#[test]
fn vec3_add_vec3() {
    let v = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(3.0, v.x);
    assert_eq!(5.0, v.y);
    assert_eq!(7.0, v.z);
}

#[test]
fn vec3_sub_vec3() {
    let v = Vec3::new(10.0, 12.0, 14.0) - Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(9.0, v.x);
    assert_eq!(10.0, v.y);
    assert_eq!(11.0, v.z);
}

#[test]
fn vec3_mul_scalar() {
    let v = Vec3::new(1.0, 2.0, 3.0) * 2.0;
    assert_eq!(2.0, v.x);
    assert_eq!(4.0, v.y);
    assert_eq!(6.0, v.z);
}

#[test]
fn scalar_mul_vec3() {
    let v = 2.0 * Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(2.0, v.x);
    assert_eq!(4.0, v.y);
    assert_eq!(6.0, v.z);
}

#[test]
fn vec3_div_scalar() {
    let v = Vec3::new(2.0, 4.0, 6.0) / 2.0;
    assert_eq!(1.0, v.x);
    assert_eq!(2.0, v.y);
    assert_eq!(3.0, v.z);
}

#[test]
fn vec3_dot() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(20.0, v1.dot(v2));
}

#[test]
fn vec3_cross() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(2.0, 3.0, 4.0);
    let a = v1.cross(v2);
    let b = v2.cross(v1);
    assert_eq!(0.0, v1.dot(a));
    assert_eq!(0.0, v2.dot(a));
    assert_eq!(0.0, (a + b).len());
}

#[test]
fn vec3_len() {
    assert_eq!(5.0, Vec3::new(3.0, 4.0, 0.0).len());
    assert_eq!(5.0, Vec3::new(0.0, 4.0, 3.0).len());
    assert_eq!(5.0, Vec3::new(3.0, 0.0, 4.0).len());
}

#[test]
fn vec3_unit() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let u = v.unit();

    assert_eq!(1.0, u.len());
    assert_eq!(0.0, u.cross(v).len());
}

#[test]
fn vec3_min_vec3() {
    let v1 = Vec3::new(1.0, 3.0, 5.0);
    let v2 = Vec3::new(2.0, 2.5, 4.0);
    let min = v1.min(v2);
    assert_eq!(1.0, min.x);
    assert_eq!(2.5, min.y);
    assert_eq!(4.0, min.z);
}

#[test]
fn vec3_max_vec3() {
    let v1 = Vec3::new(1.0, 3.0, 5.0);
    let v2 = Vec3::new(2.0, 2.5, 4.0);
    let max = v1.max(v2);
    assert_eq!(2.0, max.x);
    assert_eq!(3.0, max.y);
    assert_eq!(5.0, max.z);
}
