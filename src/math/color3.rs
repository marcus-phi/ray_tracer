use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color3 {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color3 {
    pub fn new(r: f64, g: f64, b: f64) -> Color3 {
        Color3 { r, g, b }
    }
}

impl ops::Add<Color3> for Color3 {
    type Output = Color3;

    fn add(self, c: Color3) -> Color3 {
        Color3::new(self.r + c.r, self.g + c.g, self.b + c.b)
    }
}

impl ops::Sub<Color3> for Color3 {
    type Output = Color3;

    fn sub(self, c: Color3) -> Color3 {
        Color3::new(self.r - c.r, self.g - c.g, self.b - c.b)
    }
}

impl ops::Mul<f64> for Color3 {
    type Output = Color3;

    fn mul(self, s: f64) -> Color3 {
        Color3::new(self.r * s, self.g * s, self.b * s)
    }
}

impl ops::Mul<Color3> for f64 {
    type Output = Color3;

    fn mul(self, c: Color3) -> Color3 {
        Color3::new(self * c.r, self * c.g, self * c.b)
    }
}

impl ops::Div<f64> for Color3 {
    type Output = Color3;

    fn div(self, s: f64) -> Color3 {
        Color3::new(self.r / s, self.g / s, self.b / s)
    }
}

#[test]
fn color3_add_color3() {
    let c = Color3::new(1.0, 2.0, 3.0) + Color3::new(2.0, 3.0, 4.0);
    assert_eq!(3.0, c.r);
    assert_eq!(5.0, c.g);
    assert_eq!(7.0, c.b);
}

#[test]
fn color3_sub_color3() {
    let c = Color3::new(10.0, 12.0, 14.0) - Color3::new(1.0, 2.0, 3.0);
    assert_eq!(9.0, c.r);
    assert_eq!(10.0, c.g);
    assert_eq!(11.0, c.b);
}

#[test]
fn color3_mul_scalar() {
    let c = Color3::new(1.0, 2.0, 3.0) * 2.0;
    assert_eq!(2.0, c.r);
    assert_eq!(4.0, c.g);
    assert_eq!(6.0, c.b);
}

#[test]
fn scalar_mul_color3() {
    let c = 2.0 * Color3::new(1.0, 2.0, 3.0);
    assert_eq!(2.0, c.r);
    assert_eq!(4.0, c.g);
    assert_eq!(6.0, c.b);
}

#[test]
fn color3_div_scalar() {
    let c = Color3::new(2.0, 4.0, 6.0) / 2.0;
    assert_eq!(1.0, c.r);
    assert_eq!(2.0, c.g);
    assert_eq!(3.0, c.b);
}
