pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use light::Light;
pub use metal::Metal;

use super::ray::{Color, HitRecord, Ray};

pub struct ScatterResult {
    pub attenuation: Color,
    pub luminosity: f32,
    pub new_ray: Option<Ray>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<ScatterResult>;
}
