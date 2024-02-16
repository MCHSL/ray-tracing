use std::sync::Arc;

use crate::{
    math::Interval,
    rendering::ray::{HitRecord, Ray},
};

use super::{aabb::Aabb, Object};

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
