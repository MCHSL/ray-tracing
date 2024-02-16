use std::f32::consts::PI;

use glam::{vec3, Vec3};

use crate::{
    math::Interval,
    object::{aabb::Aabb, Object},
    rendering::{
        material::Material,
        ray::{HitRecord, Ray},
    },
};

pub struct Sphere<M: Material> {
    pub start_center: Vec3,
    pub movement_vector: Vec3,
    pub radius: f32,
    pub material: M,
    pub bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Self {
        let rbox = vec3(radius, radius, radius);
        Self {
            start_center: center,
            movement_vector: Vec3::ZERO,
            radius,
            material,
            bbox: Aabb::from_points(center - rbox, center + rbox),
        }
    }

    pub fn moving(start: Vec3, end: Vec3, radius: f32, material: M) -> Self {
        let rbox = vec3(radius, radius, radius);
        let start_box = Aabb::from_points(start - rbox, start + rbox);
        let end_box = Aabb::from_points(end - rbox, end + rbox);
        Self {
            start_center: start,
            movement_vector: end - start,
            radius,
            material,
            bbox: Aabb::from_boxes(&start_box, &end_box),
        }
    }

    fn center(&self, time: f32) -> Vec3 {
        self.start_center + time * self.movement_vector
    }

    fn get_uv(&self, point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl<M: Material> Object for Sphere<M> {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center(ray.time)) / self.radius;
        let (u, v) = self.get_uv(normal);

        Some(HitRecord::new(
            self,
            ray,
            point,
            normal,
            root,
            &self.material,
            u,
            v,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn position(&self) -> glam::Vec3 {
        self.start_center
    }
}
