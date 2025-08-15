pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (1.0 - t) * start + end * t
}
