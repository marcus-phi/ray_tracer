use crate::math::Color3;

pub trait Texture {
    fn value(&self, u: f64, v: f64) -> Color3;
}

#[derive(Debug, Copy, Clone)]
pub struct ConstantTexture {
    pub color: Color3,
}

impl ConstantTexture {
    pub fn new(color: Color3) -> ConstantTexture {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f64, _: f64) -> Color3 {
        self.color
    }
}

#[test]
fn constant_texture() {
    let c = Color3::new(1.0, 2.0, 3.0);
    let t = ConstantTexture::new(c);
    let v = t.value(0.5, 0.6);

    assert_eq!(1.0, t.color.r);
    assert_eq!(2.0, t.color.g);
    assert_eq!(3.0, t.color.b);
    assert_eq!(1.0, v.r);
    assert_eq!(2.0, v.g);
    assert_eq!(3.0, v.b);
}
