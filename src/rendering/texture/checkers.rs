use glam::Vec3;

use crate::rendering::ray::Color;

use super::{solid::SolidColor, Texture};

pub struct CheckerTexture<T: Texture, U: Texture> {
    inverse_scale: f32,
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(scale: f32, even: T, odd: U) -> Self {
        Self {
            inverse_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn with_colors(scale: f32, even: Color, odd: Color) -> Self {
        Self::new(scale, SolidColor::new(even), SolidColor::new(odd))
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Color {
        let floored = (self.inverse_scale * point).floor();
        let is_even = (floored.x + floored.y + floored.z) as i32 % 2 == 0;
        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}
