use crate::rendering::{
    ray::{Color, HitRecord, Ray},
    texture::{solid::SolidColor, Texture},
};

use super::{Material, ScatterResult};

pub struct Light<T: Texture> {
    albedo: T,
}

impl<T: Texture> Light<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}

impl Light<SolidColor> {
    pub fn solid_color(color: Color) -> Self {
        Self::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for Light<T> {
    fn scatter(&self, _incoming: Ray, _hit: &HitRecord) -> Option<ScatterResult> {
        None
    }

    fn emit(&self, u: f32, v: f32, point: glam::Vec3) -> Color {
        self.albedo.value(u, v, point)
    }
}
