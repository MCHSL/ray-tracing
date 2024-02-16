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
    fn scatter(&self, _incoming: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.albedo.value(hit.u, hit.v, hit.point),
            luminosity: 64.0,
            new_ray: None,
        })
    }
}
