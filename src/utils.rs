pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + end * t
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.min(self.max).max(self.min)
    }
}
