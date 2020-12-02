use crate::math::Color3;
use crate::texture::Texture;

#[derive(Debug, Copy, Clone)]
pub struct ConstantTexture {
    color: Color3,
}

impl ConstantTexture {
    pub fn new(color: Color3) -> Box<ConstantTexture> {
        Box::new(ConstantTexture { color })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f64, _: f64) -> Color3 {
        self.color
    }
}

#[test]
fn constant_texture() {
    let t = ConstantTexture::new(Color3::new(1.0, 2.0, 3.0));
    let v = t.value(0.5, 0.6);

    assert_eq!(1.0, v.r);
    assert_eq!(2.0, v.g);
    assert_eq!(3.0, v.b);
}
