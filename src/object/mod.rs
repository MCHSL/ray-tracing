pub mod aabb;
pub mod bvh;
pub mod collection;
pub mod types;

use crate::{
    math::Interval,
    rendering::ray::{HitRecord, Ray},
};

use self::aabb::Aabb;

pub trait Object: Send + Sync {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Aabb;
}
