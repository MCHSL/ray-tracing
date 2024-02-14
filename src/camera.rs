use glam::{vec3, Vec3};
use image::{ImageBuffer, Rgb};
use rand::distributions::{Distribution, Uniform};
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{
    object::Object,
    ray::{Color, Ray},
    vector::VecExt,
};

pub struct CameraConfig {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
    pub vfov: f32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vector_up: Vec3,
    pub focus_distance: f32,
    pub defocus_angle: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 10,
            max_bounces: 50,
            vfov: 90.0,
            look_from: vec3(0.0, 0.0, 0.0),
            look_at: vec3(0.0, 0.0, 2.0),
            vector_up: vec3(0.0, 1.0, 0.0),
            focus_distance: 10.0,
            defocus_angle: 0.0,
        }
    }
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_bounces: u32,

    center: Vec3,
    pixel00_loc: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

fn linear_to_gamma(linear: f32) -> f32 {
    linear.sqrt()
}

impl Camera {
    pub fn new(config: CameraConfig) -> Self {
        let CameraConfig {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_bounces,
            vfov,
            look_from,
            look_at,
            vector_up,
            focus_distance,
            defocus_angle,
        } = config;

        let image_height = (image_width as f32 / aspect_ratio) as u32;

        let center = look_from;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        let w = (look_from - look_at).normalize();
        let u = vector_up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left =
            center - (focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_distance * ((defocus_angle / 2.0).to_radians().tan());
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            samples_per_pixel,
            max_bounces,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn create_ray(&self, center_x: f32, center_y: f32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (center_x * self.pixel_delta_u) + (center_y * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let point = Vec3::random_in_unit_disk();
        self.center + point.x * self.defocus_disk_u + point.y * self.defocus_disk_v
    }

    fn pixel_sample_square(&self) -> Vec3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(-0.5..0.5);
        let px = uniform.sample(&mut rng);
        let py = uniform.sample(&mut rng);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color<T: Object>(ray: Ray, bounces_left: u32, hittable: &T) -> Color {
        if bounces_left == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = hittable.hit(ray, &(0.001..f32::MAX)) {
            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                return Color::from(
                    scatter.attenuation.0
                        * Self::ray_color(scatter.new_ray, bounces_left - 1, hittable).0,
                );
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }

    pub fn render<T: Object + Sync>(&self, world: &T) -> image::RgbImage {
        let mut buffer: image::RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        buffer
            .enumerate_rows_mut()
            .par_bridge()
            .for_each(|(row_index, row)| {
                row.for_each(|(x, y, pixel)| {
                    let mut c = Vec3::ZERO;
                    for _ in 0..self.samples_per_pixel {
                        let ray = self.create_ray(x as f32, y as f32);
                        c += Self::ray_color(ray, self.max_bounces, world).0;
                    }

                    let c = c / self.samples_per_pixel as f32;

                    *pixel = Rgb([
                        (linear_to_gamma(c.x) * 255.0) as u8,
                        (linear_to_gamma(c.y) * 255.0) as u8,
                        (linear_to_gamma(c.z) * 255.0) as u8,
                    ]);
                });

                print!(
                    "\rRendering... {:.0}%",
                    (row_index as f32 / self.image_height as f32) * 100.0
                );
            });

        buffer
    }
}
