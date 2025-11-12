use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // ∅ ∪ X = X -> union between empty set and X should return X
    // ∅ ∩ X = ∅ -> intersection between empty set and X should return empty set
    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn unit() -> Self {
        Self { min: 0.0, max: 1.0 }
    }

    pub fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn expand(mut self, delta: f64) -> Self {
        let padding = delta / 2.0;

        self.min = self.min - padding;
        self.max = self.max + padding;
        self
    }

    pub fn union(a: &Interval, b: &Interval) -> Self {
        Self {
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

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}
