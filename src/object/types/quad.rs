use glam::Vec3;

use crate::{
    math::Interval,
    object::{aabb::Aabb, Object},
    rendering::{
        material::Material,
        ray::{HitRecord, Ray},
    },
};

pub struct Quad<M: Material> {
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    material: M,
    bbox: Aabb,
    d_component: f32,
    w_component: Vec3,
    normal: Vec3,
}

impl<M: Material> Quad<M> {
    pub fn new(origin: Vec3, u: Vec3, v: Vec3, material: M) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        Self {
            origin,
            u,
            v,
            material,
            bbox: Aabb::from_points(origin, origin + u + v).pad(),
            normal,
            d_component: normal.dot(origin),
            w_component: n / n.dot(n),
        }
    }
}

impl<M: Material> Object for Quad<M> {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord> {
        let denominator = self.normal.dot(ray.direction);

        if denominator.abs() < 1e-8_f32 {
            return None;
        }

        let t = (self.d_component - self.normal.dot(ray.origin)) / denominator;
        if !range.contains(&t) {
            return None;
        }

        let intersection = ray.at(t);

        let hit_vector = intersection - self.origin;
        let alpha = self.w_component.dot(hit_vector.cross(self.v));
        let beta = self.w_component.dot(self.u.cross(hit_vector));

        if !(0.0..1.0).contains(&alpha) || !(0.0..1.0).contains(&beta) {
            return None;
        }

        Some(HitRecord::new(
            self,
            ray,
            intersection,
            self.normal,
            t,
            &self.material,
            alpha,
            beta,
        ))
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn position(&self) -> glam::Vec3 {
        self.origin + (self.u * 0.5) + (self.v * 0.5)
    }
}
