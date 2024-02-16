use std::path::Path;

use glam::Vec3;
use image::io::Reader as ImageReader;
use image::RgbImage;

use crate::rendering::ray::Color;

use super::Texture;

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        Self {
            image: RgbImage::from(img),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: Vec3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = u * (self.image.width() - 1) as f32;
        let j = v * (self.image.height() - 1) as f32;

        let pixel = self.image.get_pixel(i as u32, j as u32);

        let scale = 1. / 255.;
        Color::new(
            pixel[0] as f32 * scale,
            pixel[1] as f32 * scale,
            pixel[2] as f32 * scale,
        )
    }
}
