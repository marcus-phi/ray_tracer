use crate::math::Color3;

pub trait Texture {
    fn value(&self, u: f64, v: f64) -> Color3;
}

mod constant_texture;
pub use constant_texture::ConstantTexture;
