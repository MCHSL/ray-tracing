use glam::Vec3;

use crate::{
    math::VecExt,
    rendering::{
        ray::{Color, HitRecord, Ray},
        texture::{solid::SolidColor, Texture},
    },
};

use super::{Material, ScatterResult};

pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Self { albedo: texture }
    }
}

impl Lambertian<SolidColor> {
    pub fn solid_color(color: Color) -> Self {
        Self::new(SolidColor::new(color))
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let mut direction = hit.normal + Vec3::random_unit();
        if direction.near_zero() {
            direction = hit.normal;
        }
        let scattered = Ray::new(hit.point, direction, incoming.time);
        Some(ScatterResult {
            attenuation: self.albedo.value(hit.u, hit.v, hit.point),
            luminosity: 1.0,
            new_ray: Some(scattered),
        })
    }
}
