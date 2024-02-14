use glam::{vec3, Vec3};
use rand::distributions::{Distribution, Uniform};

use crate::{common::Interval, math::random_range};

pub trait VecExt {
    fn random() -> Self;
    fn random_range(range: Interval) -> Self;
    fn random_in_unit_sphere() -> Self;
    fn random_unit() -> Self;
    fn random_in_unit_disk() -> Self;
    fn near_zero(&self) -> bool;
    fn reflect(&self, normal: Self) -> Self;
    fn refract(&self, normal: Self, refraction_index: f32) -> Self;
}

impl VecExt for Vec3 {
    fn random() -> Vec3 {
        Self::random_range(0.0..1.0)
    }

    fn random_range(range: Interval) -> Vec3 {
        let mut rng = rand::thread_rng();
        let uniform = Uniform::from(range);
        let x = uniform.sample(&mut rng);
        let y = uniform.sample(&mut rng);
        let z = uniform.sample(&mut rng);
        vec3(x, y, z)
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Self::random_range(-1.0..1.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    fn random_unit() -> Vec3 {
        Self::random_in_unit_sphere().normalize()
    }

    fn random_in_unit_disk() -> Self {
        loop {
            let v = vec3(random_range(-1.0..0.0), random_range(-1.0..0.0), 0.0);
            if v.length_squared() < 1.0 {
                return v;
            }
        }
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8_f32;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    fn refract(&self, normal: Self, refraction_index: f32) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = refraction_index * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}
