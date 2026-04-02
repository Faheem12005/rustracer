use crate::utils::INFINITY;

pub struct Interval {
    pub max: f64,
    pub min: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }
    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }
}

pub const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);
pub const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);