use std::ops::Range;

pub type Interval = Range<f32>;

pub trait IntervalExt {
    fn from_intervals(a: &Self, b: &Self) -> Self;
}

impl IntervalExt for Interval {
    fn from_intervals(a: &Self, b: &Self) -> Self {
        a.start.min(b.start)..a.end.max(b.end)
    }
}
