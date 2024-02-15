use glam::Vec3;

use crate::{
    math::random,
    ray::{Color, HitRecord, Ray},
    texture::{SolidColor, Texture},
    vector::VecExt,
};

pub struct ScatterResult {
    pub attenuation: Color,
    pub luminosity: f32,
    pub new_ray: Option<Ray>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<ScatterResult>;
}

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
            luminosity: 1.0,
            new_ray: Some(new_ray),
        })
    }
}

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
