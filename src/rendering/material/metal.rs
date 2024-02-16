use glam::Vec3;

use crate::{
    math::VecExt,
    rendering::{
        ray::{Color, HitRecord, Ray},
        texture::{solid::SolidColor, Texture},
    },
};

use super::{Material, ScatterResult};

pub struct Metal<T: Texture> {
    pub albedo: T,
    pub fuzz: f32,
}

impl<T: Texture> Metal<T> {
    pub fn new(texture: T, fuzz: f32) -> Self {
        Self {
            albedo: texture,
            fuzz,
        }
    }
}

impl Metal<SolidColor> {
    pub fn solid_color(color: Color, fuzz: f32) -> Self {
        Self::new(SolidColor::new(color), fuzz)
    }
}

impl<T: Texture> Material for Metal<T> {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflection_direction = incoming.direction.normalize().reflect(hit.normal);
        let new_ray = Ray::new(
            hit.point,
            reflection_direction + self.fuzz * Vec3::random_unit(),
            incoming.time,
        );
        Some(ScatterResult {
            attenuation: self.albedo.value(hit.u, hit.v, hit.point),
            new_ray: Some(new_ray),
        })
    }
}
