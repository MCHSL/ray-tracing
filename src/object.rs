use std::{ops::Range, sync::Arc};

use crate::{
    material::Material,
    ray::{HitRecord, Ray},
};
use glam::Vec3;

pub trait Object: Send + Sync {
    fn hit(&self, ray: Ray, range: &Range<f32>) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Object for Sphere {
    fn hit(&self, ray: Ray, range: &Range<f32>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
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
        let normal = (point - self.center) / self.radius;

        Some(HitRecord::new(
            ray,
            point,
            normal,
            root,
            self.material.as_ref(),
        ))
    }
}

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl Object for World {
    fn hit(&self, ray: Ray, range: &Range<f32>) -> Option<HitRecord> {
        use ord_subset::OrdSubsetIterExt;

        self.objects
            .iter()
            .filter_map(|o| o.hit(ray, range))
            .ord_subset_min_by_key(|o| o.t)
    }
}

impl Object for &World {
    fn hit(&self, ray: Ray, range: &Range<f32>) -> Option<HitRecord> {
        (*self).hit(ray, range)
    }
}
