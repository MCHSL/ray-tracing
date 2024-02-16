use std::sync::Arc;

use glam::vec3;

use crate::{
    math::Interval,
    rendering::ray::{HitRecord, Ray},
};

use super::{
    aabb::Aabb,
    bvh::{BVHCollection, BVHNode},
    Object,
};

pub struct ObjectCollection {
    objects: Vec<Arc<dyn Object>>,
    lights: Vec<Arc<dyn Object>>,
    bbox: Aabb,
}

impl ObjectCollection {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            bbox: Aabb::empty(),
        }
    }

    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        self.bbox = Aabb::from_boxes(&self.bbox, obj.bounding_box());
        self.objects.push(Arc::new(obj));
    }

    pub fn add_light<T: Object + 'static>(&mut self, obj: T) {
        self.bbox = Aabb::from_boxes(&self.bbox, obj.bounding_box());
        let o = Arc::new(obj);
        self.objects.push(o.clone());
        self.lights.push(o);
    }

    pub fn objects(&self) -> &Vec<Arc<dyn Object>> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<Arc<dyn Object>> {
        &self.lights
    }

    pub fn as_bvh(&self) -> BVHCollection {
        BVHCollection::from_simple_collection(self)
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

    fn position(&self) -> glam::Vec3 {
        vec3(0., 0., 0.)
    }
}

impl Object for &ObjectCollection {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord> {
        (*self).hit(ray, range)
    }

    fn bounding_box(&self) -> &Aabb {
        (*self).bounding_box()
    }

    fn position(&self) -> glam::Vec3 {
        vec3(0., 0., 0.)
    }
}
