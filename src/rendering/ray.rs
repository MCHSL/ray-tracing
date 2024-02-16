use derive_more::{Add, From, Mul};
use glam::Vec3;

use crate::math::VecExt;

use super::material::Material;

#[derive(Clone, Copy, From, Add, Mul)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3 { x, y, z })
    }

    pub fn random() -> Self {
        Self(Vec3::random())
    }
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub t: f32,
    pub front_face: bool,
    pub u: f32,
    pub v: f32,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        ray: Ray,
        point: Vec3,
        outward_normal: Vec3,
        t: f32,
        material: &'a dyn Material,
        u: f32,
        v: f32,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            material,
            t,
            front_face,
            u,
            v,
        }
    }
}
