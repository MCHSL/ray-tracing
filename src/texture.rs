use std::path::Path;

use glam::Vec3;
use image::io::Reader as ImageReader;
use image::RgbImage;

use crate::ray::Color;

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture<T: Texture, U: Texture> {
    inverse_scale: f32,
    even: T,
    odd: U,
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(scale: f32, even: T, odd: U) -> Self {
        Self {
            inverse_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn with_colors(scale: f32, even: Color, odd: Color) -> Self {
        Self::new(scale, SolidColor::new(even), SolidColor::new(odd))
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn value(&self, u: f32, v: f32, point: Vec3) -> Color {
        let floored = (self.inverse_scale * point).floor();
        let is_even = (floored.x + floored.y + floored.z) as i32 % 2 == 0;
        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

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
