use std::{cmp::Ordering, sync::Arc};

use crate::{
    math::{random_int, Interval},
    object::Object,
    rendering::ray::{HitRecord, Ray},
};

use super::aabb::Aabb;

pub struct BVHNode {
    left: Arc<dyn Object>,
    right: Arc<dyn Object>,
    bbox: Aabb,
}

impl BVHNode {
    pub fn new(list: &Vec<Arc<dyn Object>>) -> Self {
        let len = list.len();
        Self::from_object_range(list, 0, len)
    }

    pub fn from_object_range(source: &Vec<Arc<dyn Object>>, start: usize, end: usize) -> Self {
        let axis = random_int(0..3);
        let comparator = match axis {
            0 => Self::compare_boxes_x,
            1 => Self::compare_boxes_y,
            2 => Self::compare_boxes_z,
            _ => unreachable!(),
        };

        let span = end - start;
        let (left, right) = match span {
            1 => {
                let obj = source.get(start).cloned().unwrap();
                (obj.clone(), obj)
            }
            2 => {
                let one = source.get(start).cloned().unwrap();
                let two = source.get(start + 1).cloned().unwrap();
                if comparator(&one, &two) == Ordering::Less {
                    (one, two)
                } else {
                    (two, one)
                }
            }
            _ => {
                let mut needed_part = source[start..end].to_vec();
                needed_part.sort_by(comparator);

                let mid = (start + end) / 2;
                let left = Self::from_object_range(source, start, mid);
                let right = Self::from_object_range(source, mid, end);
                (
                    Arc::new(left) as Arc<dyn Object>,
                    Arc::new(right) as Arc<dyn Object>,
                )
            }
        };

        let bbox = Aabb::from_boxes(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }

    fn compare_boxes(a: &Arc<dyn Object>, b: &Arc<dyn Object>, axis: usize) -> Ordering {
        a.bounding_box()
            .axis(axis)
            .start
            .total_cmp(&b.bounding_box().axis(axis).start)
    }

    fn compare_boxes_x(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
        Self::compare_boxes(a, b, 0)
    }

    fn compare_boxes_y(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
        Self::compare_boxes(a, b, 1)
    }

    fn compare_boxes_z(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
        Self::compare_boxes(a, b, 2)
    }
}

impl Object for BVHNode {
    fn hit(&self, ray: Ray, range: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, range) {
            return None;
        }

        let left_hit = self.left.hit(ray, range);

        let new_range = if let Some(ref hit) = left_hit {
            range.start..hit.t
        } else {
            range.clone()
        };
        let right_hit = self.right.hit(ray, &new_range);

        right_hit.or(left_hit)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
