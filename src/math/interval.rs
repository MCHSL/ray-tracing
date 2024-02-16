use std::ops::Range;

pub type Interval = Range<f32>;

pub trait IntervalExt {
    fn from_intervals(a: &Self, b: &Self) -> Self;
    fn expand(&self, by: f32) -> Self;
    fn size(&self) -> f32;
}

impl IntervalExt for Interval {
    fn from_intervals(a: &Self, b: &Self) -> Self {
        a.start.min(b.start)..a.end.max(b.end)
    }

    fn expand(&self, by: f32) -> Self {
        (self.start - by)..(self.start + by)
    }

    fn size(&self) -> f32 {
        self.end - self.start
    }
}
