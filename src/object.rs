use std::{f32::consts::PI, sync::Arc};

use crate::{
    aabb::Aabb,
    common::Interval,
    material::Material,
    ray::{HitRecord, Ray},
};
use glam::{vec3, Vec3};

pub trait Object: Send + Sync {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}

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
}

pub struct ObjectCollection {
    objects: Vec<Arc<dyn Object>>,
    bbox: Aabb,
}

impl ObjectCollection {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::empty(),
        }
    }

    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        self.bbox = Aabb::from_boxes(&self.bbox, obj.bounding_box());
        self.objects.push(Arc::new(obj));
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Object>> {
        &self.objects
    }
}

impl Object for ObjectCollection {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord> {
        use ord_subset::OrdSubsetIterExt;

        self.objects
            .iter()
            .filter_map(|o| o.hit(ray, range))
            .ord_subset_min_by_key(|o| o.t)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl Object for &ObjectCollection {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord> {
        (*self).hit(ray, range)
    }

    fn bounding_box(&self) -> &Aabb {
        (*self).bounding_box()
    }
}
