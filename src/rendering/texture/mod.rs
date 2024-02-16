pub mod checkers;
pub mod image;
pub mod solid;

pub use checkers::CheckerTexture;
pub use image::ImageTexture;
pub use solid::SolidColor;

use glam::Vec3;

use super::ray::Color;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Color;
}
