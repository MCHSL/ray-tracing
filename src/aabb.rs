use glam::Vec3;

use crate::{
    common::{Interval, IntervalExt},
    ray::Ray,
};

#[derive(Debug, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn empty() -> Self {
        Self {
            x: f32::MAX..f32::MIN,
            y: f32::MAX..f32::MIN,
            z: f32::MAX..f32::MIN,
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        Self {
            x: a.x.min(b.x)..a.x.max(b.x),
            y: a.y.min(b.y)..a.y.max(b.y),
            z: a.z.min(b.z)..a.z.max(b.z),
        }
    }

    pub fn from_boxes(a: &Self, b: &Self) -> Self {
        Self {
            x: Interval::from_intervals(&a.x, &b.x),
            y: Interval::from_intervals(&a.y, &b.y),
            z: Interval::from_intervals(&a.z, &b.z),
        }
    }

    pub fn axis(&self, a: usize) -> &Interval {
        match a {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, ray: Ray, range: &Interval) -> bool {
        let mut range = range.to_owned();
        for a in 0..3 {
            let inverse_direction = (1.0 / ray.direction)[a];
            let origin = ray.origin[a];

            let mut t0 = (self.axis(a).start - origin) * inverse_direction;
            let mut t1 = (self.axis(a).end - origin) * inverse_direction;

            if inverse_direction < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > range.start {
                range.start = t0;
            }
            if t1 < range.end {
                range.end = t1;
            }

            if range.end < range.start {
                return false;
            }
        }

        true
    }
}
