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
    pub fn new(color: Color) -> Box<Self> {
        Box::new(Self { color })
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: Vec3) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    inverse_scale: f32,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Color, odd: Color) -> Box<Self> {
        Box::new(Self {
            inverse_scale: 1.0 / scale,
            even: SolidColor::new(even),
            odd: SolidColor::new(odd),
        })
    }
}

impl Texture for CheckerTexture {
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
    pub fn from_file<P: AsRef<Path>>(path: P) -> Box<Self> {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        Box::new(Self {
            image: RgbImage::from(img),
        })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: Vec3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = u * self.image.width() as f32;
        let j = v * self.image.height() as f32;

        let pixel = self.image.get_pixel(i as u32, j as u32);

        let scale = 1. / 255.;
        Color::new(
            pixel[0] as f32 * scale,
            pixel[1] as f32 * scale,
            pixel[2] as f32 * scale,
        )
    }
}
