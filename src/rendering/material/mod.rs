pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;

pub use dielectric::Dielectric;
use glam::Vec3;
pub use lambertian::Lambertian;
pub use light::Light;
pub use metal::Metal;

use super::ray::{Color, HitRecord, Ray};

pub struct ScatterResult {
    pub attenuation: Color,
    pub new_ray: Option<Ray>,
}

pub trait Material: Send + Sync {
    fn scatter(&self, incoming: Ray, hit: &HitRecord) -> Option<ScatterResult>;
    fn emit(&self, _u: f32, _v: f32, _point: Vec3) -> Color {
        Color::new(0., 0., 0.)
    }
}
