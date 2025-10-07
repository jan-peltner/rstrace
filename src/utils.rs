pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + end * t
}

pub fn linear_to_gamma(val: f64) -> f64 {
    if val <= 0.0 {
        return 0.0;
    }
    val.sqrt()
}

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn empty() -> Self {
        Interval { min: 0.0, max: 0.0 }
    }

    pub fn expand(mut self, delta: f64) -> Self {
        let padding = delta / 2.0;

        self.min = self.min - padding;
        self.max = self.max + padding;
        self
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

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
