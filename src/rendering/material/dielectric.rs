use crate::{
    math::{random, VecExt},
    rendering::ray::{Color, HitRecord, Ray},
};

use super::{Material, ScatterResult};

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Dielectric {
    fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = incoming.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                unit_direction.reflect(hit.normal)
            } else {
                unit_direction.refract(hit.normal, refraction_ratio)
            };

        let new_ray = Ray::new(hit.point, direction, incoming.time);

        Some(ScatterResult {
            attenuation: Color::new(1.0, 1.0, 1.0),
            luminosity: 1.0,
            new_ray: Some(new_ray),
        })
    }
}
