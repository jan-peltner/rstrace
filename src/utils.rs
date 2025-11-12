use core::f64;

use crate::interval::Interval;

pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + end * t
}

pub fn linear_to_gamma(val: f64) -> f64 {
    if val <= 0.0 {
        return 0.0;
    }
    val.sqrt()
}

const RGB_INTERVAL: Interval = Interval {
    min: 0.0,
    max: 255.99,
};

pub fn map_rgb(val: f64) -> f64 {
    RGB_INTERVAL.clamp(val * RGB_INTERVAL.max)
}
